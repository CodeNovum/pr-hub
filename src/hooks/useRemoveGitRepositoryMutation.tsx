import {
  COMMAND_REMOVE_REPOSITORY,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
import { useStoreActions } from "../store/store";
import {
  UseMutationResult,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

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

  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useMutation({
    mutationFn: async (gitRepositoryId: number) => {
      try {
        await invoke(COMMAND_REMOVE_REPOSITORY, { id: gitRepositoryId });
        updateGlobalNotificationMessage({
          message: "Repository was removed",
          type: "Success",
        });
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Repository could not be removed",
          type: "Error",
        });
      }
    },
    onSettled: () =>
      queryClient.invalidateQueries({
        queryKey: [RQ_KEY_IMPORTED_GIT_REPOSITORIES],
      }),
  });
};

export { useRemoveGitRepositoryMutation };
