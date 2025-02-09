import { GitRepository } from "../bindings/devops";
import { activeRepositoriesLocalStorageKey } from "../constants";
import { useStoreActions } from "../store/store";
import { UseQueryResult, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to retrieve the list of imported repositories marked
 * as active based on the current app state.
 *
 * @returns {UseQueryResult<GitRepository[]>} The query result
 */
const useRepositories = (): UseQueryResult<GitRepository[]> => {
  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useQuery({
    queryKey: ["devops-repositories"],
    queryFn: async () => {
      try {
        // Fetch the imported repositories for the user.
        const result = await invoke<GitRepository[]>("get_repositories");
        // Hydrate the repository status regarding to the filter.
        const storedRepositoryIdsForGlobalFilter = localStorage.getItem(
          activeRepositoriesLocalStorageKey,
        );
        if (storedRepositoryIdsForGlobalFilter) {
          // There is a stored filter. Apply it.
          const parsedRepositoryIdsForGlobalFilter: string[] = JSON.parse(
            storedRepositoryIdsForGlobalFilter,
          );
          result.forEach((repo) => {
            repo.isActive =
              repo.id && parsedRepositoryIdsForGlobalFilter.includes(repo.id)
                ? true
                : false;
          });
        } else {
          // There is no filter stored. Set all repositories to active.
          result.forEach((repo) => (repo.isActive = true));
        }
        return result;
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Could not retrieve repositories",
          type: "Error",
        });
        return [];
      }
    },
    staleTime: 100000,
  });
};

export { useRepositories };
