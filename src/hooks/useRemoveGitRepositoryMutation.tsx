import {
  COMMAND_REMOVE_REPOSITORY,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
import {
  UseMutationResult,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "react-toastify";

/**
 * Hook to remove a single imported git repository 
 *
 * @returns {UseMutationResult<
 void,
 null,
 number
>} The mutation result
 */
const useRemoveGitRepositoryMutation = (): UseMutationResult<
  void,
  null,
  number
> => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (gitRepositoryId: number) => {
      try {
        await invoke(COMMAND_REMOVE_REPOSITORY, { id: gitRepositoryId });
        toast("Repository was removed", { type: "success" });
      } catch (error) {
        console.error(error);
        toast("Repository could not be removed", { type: "error" });
      }
    },
    onSettled: () =>
      queryClient.invalidateQueries({
        queryKey: [RQ_KEY_IMPORTED_GIT_REPOSITORIES],
      }),
  });
};

export { useRemoveGitRepositoryMutation };
