import { GitRepositoryDto } from "../bindings";
import {
  COMMAND_GET_REPOSITORIES,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
import { useStoreActions } from "../store/store";
import { UseQueryResult, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to retrieve the list of imported repositories
 *
 * @returns {UseQueryResult<GitRepositoryDta[]>} The query result
 */
const useRepositories = (): UseQueryResult<GitRepositoryDto[]> => {
  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useQuery({
    queryKey: [RQ_KEY_IMPORTED_GIT_REPOSITORIES],
    queryFn: async () => {
      try {
        const result = await invoke<GitRepositoryDto[]>(
          COMMAND_GET_REPOSITORIES,
        );
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
