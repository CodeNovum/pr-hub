import { GitRepository } from "../../bindings/devops";
import { ACTIVE_REPOS_LOCAL_STORAGE_KEY } from "../../constants";
import { useRepositories } from "../../hooks/useRepositories";
import { Panel } from "../Panel";
import Table from "../Table";
import { ColumnDef } from "@tanstack/react-table";
import { useCallback, useEffect, useMemo, useState } from "react";

interface IGlobalProjectFilterPanelProps {
  isOpen: boolean;
  close: () => void;
}

/**
 * Specialized panel that lets the user filter through imported repositories.
 * Only the checked repositories will be used to fetch data, so reducing the
 * number can keep the UI clear and improve fetch performance.
 */
const GlobalRepositoriesFilterPanel = (
  props: IGlobalProjectFilterPanelProps,
) => {
  const repositoriesQuery = useRepositories();
  const [checkedRepositories, setCheckedRepositories] =
    useState<GitRepository[]>();

  /**
   * Callback for updating the checked repositories.
   * This will also update the local storage.
   *
   * @param {GitRepository[]} repos The checked repositories.
   */
  const updateCheckedRepositories = (repos: GitRepository[]) => {
    const selectedRepositoryIds = repos.map((checkedRepo) => checkedRepo.id);
    localStorage.setItem(
      ACTIVE_REPOS_LOCAL_STORAGE_KEY,
      JSON.stringify(selectedRepositoryIds),
    );
    setCheckedRepositories([...repos]);
  };

  useEffect(() => {
    if (!checkedRepositories && repositoriesQuery.data) {
      setCheckedRepositories(
        repositoriesQuery.data.filter((project) => project.isActive),
      );
    }
  }, [checkedRepositories, repositoriesQuery.data]);

  const formattedRepositories = useMemo(() => {
    const repos = repositoriesQuery.data;
    repos?.sort((a, b) => {
      const unifiedNameA = `${a.project?.name ?? ""} - ${a.name}`;
      const unifiedNameB = `${b.project?.name ?? ""} - ${b.name}`;
      return unifiedNameA.localeCompare(unifiedNameB);
    });
    return repos;
  }, [repositoriesQuery.data]);

  const tableColumns: ColumnDef<GitRepository>[] = useMemo(
    () => [
      {
        id: "project",
        header: "",
        accessorKey: "project.name",
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
        updateCheckedItems={(newValue) => updateCheckedRepositories(newValue)}
        columns={tableColumns}
        data={formattedRepositories}
        checkedItems={checkedRepositories ?? []}
        onRowClick={(repo) => {
          const tmpCheckedRepos = checkedRepositories
            ? [...checkedRepositories]
            : [];
          const existingIndex = tmpCheckedRepos.findIndex(
            (checkedRepo) => checkedRepo.id === repo.id,
          );
          if (existingIndex === -1) {
            tmpCheckedRepos.push(repo);
          } else {
            tmpCheckedRepos.splice(existingIndex, 1);
          }
          updateCheckedRepositories(tmpCheckedRepos);
        }}
      />
    ),
    [checkedRepositories, formattedRepositories, tableColumns],
  );

  /**
   * Callback for closing the panel.
   */
  const closePanel = useCallback(() => {
    repositoriesQuery.refetch();
    props.close();
  }, [repositoriesQuery, props]);

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

export { GlobalRepositoriesFilterPanel };
