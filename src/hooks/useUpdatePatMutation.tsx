import {
  COMMAND_UPDATE_PAT,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
import { useStoreActions } from "../store/store";
import {
  UseMutationResult,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

interface IMutationParams {
  id: number;
  pat: string;
}

/**
 * Hook to update the PAT for an existing imported 
 * git repository
 *
 * @returns {UseMutationResult<
 void,
 null,
 IMutationParams
>} The mutation result
 */
const useUpdatePatMutation = (): UseMutationResult<
  void,
  null,
  IMutationParams
> => {
  const queryClient = useQueryClient();

  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useMutation({
    mutationFn: async (mutationVars: IMutationParams) => {
      try {
        await invoke(COMMAND_UPDATE_PAT, {
          id: mutationVars.id,
          pat: mutationVars.pat,
        });
        updateGlobalNotificationMessage({
          message: "Updated the Personal Access Token",
          type: "Success",
        });
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Could not update the Personal Access Token",
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

export { useUpdatePatMutation };
