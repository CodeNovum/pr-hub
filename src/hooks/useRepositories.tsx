import { GitRepositoryDto } from "../bindings";
import {
  COMMAND_GET_REPOSITORIES,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
import { UseQueryResult, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "react-toastify";

/**
 * Hook to retrieve the list of imported repositories
 *
 * @returns {UseQueryResult<GitRepositoryDta[]>} The query result
 */
const useRepositories = (): UseQueryResult<GitRepositoryDto[]> => {
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
        toast("Could not retrieve repositories", { type: "error" });
        return [];
      }
    },
    staleTime: 100000,
  });
};

export { useRepositories };
