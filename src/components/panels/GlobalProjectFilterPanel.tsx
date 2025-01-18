import { Project } from "../../bindings/devops";
import { useDevOpsProjects } from "../../hooks/useDevOpsProjects";
import { Panel } from "../Panel";
import Table from "../Table";
import { ColumnDef } from "@tanstack/react-table";
import { useCallback, useEffect, useMemo, useState } from "react";

interface IGlobalProjectFilterPanelProps {
  isOpen: boolean;
  close: () => void;
}

/**
 * Specialized panel that lets the user filter through imported projects.
 * Only the checked projects will be used to fetch data, so reducing the
 * number can keep the UI clear and improve fetch performance.
 */
const GlobalProjectFilterPanel = (props: IGlobalProjectFilterPanelProps) => {
  const projectsQuery = useDevOpsProjects();
  const [checkedProjects, setCheckedProjects] = useState<Project[]>();

  /**
   * Callback for updating the checked projects.
   * This will also update the local storage.
   *
   * @param {Project[]} projects The checked projects.
   */
  const updateCheckedProjects = (projects: Project[]) => {
    const selectedProjectIds = projects.map(
      (checkedProject) => checkedProject.id,
    );
    localStorage.setItem(
      "selectedProjectIds",
      JSON.stringify(selectedProjectIds),
    );
    setCheckedProjects([...projects]);
  };

  useEffect(() => {
    if (!checkedProjects && projectsQuery.data) {
      setCheckedProjects(
        projectsQuery.data.filter((project) => project.isActive),
      );
    }
  }, [checkedProjects, projectsQuery.data]);

  const formattedProjects = useMemo(() => {
    const projects = projectsQuery.data;
    projects?.sort((a, b) => {
      const unifiedNameA = `${a.organizationName} - ${a.name}`;
      const unifiedNameB = `${b.organizationName} - ${b.name}`;
      return unifiedNameA.localeCompare(unifiedNameB);
    });
    return projects;
  }, [projectsQuery.data]);

  const tableColumns: ColumnDef<Project>[] = useMemo(
    () => [
      {
        id: "organizationName",
        header: "",
        accessorKey: "organizationName",
        enableSorting: false,
      },
      {
        id: "name",
        header: "",
        accessorKey: "name",
        enableSorting: false,
      },
    ],
    [],
  );

  const body = useMemo(
    () => (
      <Table
        identifierPropertyName="id"
        updateCheckedItems={(newValue) => updateCheckedProjects(newValue)}
        columns={tableColumns}
        data={formattedProjects}
        checkedItems={checkedProjects ?? []}
        onRowClick={(project) => {
          const tmpCheckedProjects = checkedProjects
            ? [...checkedProjects]
            : [];
          const existingIndex = tmpCheckedProjects.findIndex(
            (checkedProject) => checkedProject.id === project.id,
          );
          if (existingIndex === -1) {
            tmpCheckedProjects.push(project);
          } else {
            tmpCheckedProjects.splice(existingIndex, 1);
          }
          updateCheckedProjects(tmpCheckedProjects);
        }}
      />
    ),
    [checkedProjects, formattedProjects, tableColumns],
  );

  /**
   * Callback for closing the panel.
   */
  const closePanel = useCallback(() => {
    projectsQuery.refetch();
    props.close();
  }, [projectsQuery, props]);

  return (
    <Panel
      title={"Project filter"}
      isOpen={props.isOpen}
      close={closePanel}
      content={body}
      size="Large"
    />
  );
};

export { GlobalProjectFilterPanel };
