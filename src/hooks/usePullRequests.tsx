import { PullRequestDto } from "../bindings/PullRequestDto";
import { COMMAND_GET_OPEN_PULL_REQUESTS } from "../constants";
import { UseQueryResult, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "react-toastify";

/**
 * Hook to retrieve all pull requests across imported git repositories
 * based on the current filters
 *
 * @returns {UseQueryResult<PullRequestDto[]>} The query result
 */
const usePullRequests = (): UseQueryResult<PullRequestDto[]> => {
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
        toast("Could not retrieve pull requests", { type: "error" });
        return [];
      }
    },
  });
};

export { usePullRequests };
