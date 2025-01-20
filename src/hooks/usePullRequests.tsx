import { PullRequest } from "../bindings/devops";
import { useStoreActions } from "../store/store";
import { useBatchedDevopsRequest } from "./useBatchedDevopsRequest";
import { UseQueryResult, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to retrieve all pull requests across imported organization
 * based on the current filters
 *
 * @returns {UseQueryResult<PullRequest[]>} The query result
 */
const usePullRequests = (): UseQueryResult<PullRequest[]> => {
  const requestBody = useBatchedDevopsRequest();
  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useQuery({
    queryKey: ["pullRequests", requestBody],
    queryFn: async () => {
      try {
        if (requestBody && requestBody.length > 0) {
          const result = await invoke("get_open_pull_requests_batched", {
            requestModels: requestBody,
          });
          return result;
        }
        return [];
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
