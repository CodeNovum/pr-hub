import { useQuery, UseQueryResult } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { Project } from "../bindings/devops";
import { useStoreActions } from "../store/store";

/**
 * Hook to retrieve the list of Azure DevOps projects marked
 * as active based on the current app state.
 *
 * @returns {UseQueryResult<Project[]>} The query result
 */
const useDevOpsProjects = (): UseQueryResult<Project[]> => {
  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useQuery({
    queryKey: ["devops-projects"],
    queryFn: async () => {
      try {
        // Fetch the projects for the user.
        const result = await invoke<Project[]>("get_projects");
        // Hydrate the project status regarding the filter.
        const storedProjectIdsForGlobalFilter =
          localStorage.getItem("selectedProjectIds");
        if (storedProjectIdsForGlobalFilter) {
          // There is a stored filter. Apply it.
          const parsedProjectIdsForGlobalFilter: string[] = JSON.parse(
            storedProjectIdsForGlobalFilter,
          );
          result.forEach((project) => {
            project.isActive =
              project.id && parsedProjectIdsForGlobalFilter.includes(project.id)
                ? true
                : false;
          });
        } else {
          // There is no filter stored. Set all projects to active.
          result.forEach((project) => (project.isActive = true));
        }
        return result;
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Could not retrieve projects",
          type: "Error",
        });
        return [];
      }
    },
    staleTime: 100000,
  });
};

export { useDevOpsProjects };
