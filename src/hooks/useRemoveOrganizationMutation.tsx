import { useStoreActions } from "../store/store";
import {
  UseMutationResult,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to remove a single imported organization
 *
 * @returns {UseMutationResult<
 void,
 null,
 number
>} The mutation result
 */
const useRemoveOrganizationMutation = (): UseMutationResult<
  void,
  null,
  number
> => {
  const queryClient = useQueryClient();

  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useMutation({
    mutationFn: async (organizationId: number) => {
      try {
        await invoke("remove_organization", { id: organizationId });
        updateGlobalNotificationMessage({
          message: "Organization was removed",
          type: "Success",
        });
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Organization could not be removed",
          type: "Error",
        });
      }
    },
    onSettled: () => {
      // Ensure that the query that fetches the organizations is invalidated.
      queryClient.invalidateQueries({ queryKey: ["user-organizations"] });
      queryClient.invalidateQueries({ queryKey: ["devops-projects"] });
    },
  });
};

export { useRemoveOrganizationMutation };
