import {
  COMMAND_TOGGLE_REPOSITORY_ACTIVE,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
import {
  useMutation,
  UseMutationResult,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to toggle the active state of a single imported git repository
 *
 * @returns {UseMutationResult<
 void,
 null,
 number
>} The mutation result
 */
const useToggleGitRepositoryActiveStateMutation = (): UseMutationResult<
  void,
  null,
  number
> => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (gitRepositoryId: number) => {
      try {
        await invoke(COMMAND_TOGGLE_REPOSITORY_ACTIVE, { id: gitRepositoryId });
      } catch (error) {
        console.error(error);
      }
    },
    onSettled: () =>
      queryClient.invalidateQueries({
        queryKey: [RQ_KEY_IMPORTED_GIT_REPOSITORIES],
      }),
  });
};

export { useToggleGitRepositoryActiveStateMutation };
