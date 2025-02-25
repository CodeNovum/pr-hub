import { PullRequestDto } from "../bindings";
import { COMMAND_GET_OPEN_PULL_REQUESTS } from "../constants";
import { useStoreActions } from "../store/store";
import { UseQueryResult, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to retrieve all pull requests across imported git repositories
 * based on the current filters
 *
 * @returns {UseQueryResult<PullRequestDto[]>} The query result
 */
const usePullRequests = (): UseQueryResult<PullRequestDto[]> => {
  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useQuery({
    queryKey: ["pull-requests"],
    queryFn: async () => {
      try {
        const result = await invoke<PullRequestDto[]>(
          COMMAND_GET_OPEN_PULL_REQUESTS,
        );
        return result;
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Could not retrieve pull requests",
          type: "Error",
        });
        return [];
      }
    },
  });
};

export { usePullRequests };
