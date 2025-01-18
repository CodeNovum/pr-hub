import { Organization } from "../bindings/core";
import { useStoreActions } from "../store/store";
import {
  UseMutationResult,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to update the PAT for an existing imported 
 * organization
 *
 * @returns {UseMutationResult<
 void,
 null,
 Organization
>} The mutation result
 */
const useUpdatePatMutation = (): UseMutationResult<
  void,
  null,
  Organization
> => {
  const queryClient = useQueryClient();

  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useMutation({
    mutationFn: async (organization: Organization) => {
      try {
        await invoke("update_pat", {
          id: organization.id,
          patValue: organization.pat,
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
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["user-organizations"] });
      queryClient.invalidateQueries({ queryKey: ["devops-projects"] });
    },
  });
};

export { useUpdatePatMutation };
