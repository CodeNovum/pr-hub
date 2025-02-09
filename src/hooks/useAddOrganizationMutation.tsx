import { COMMAND_ADD_ORGANIZATION } from "../constants";
import { useStoreActions } from "../store/store";
import {
  UseMutationResult,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

interface IMutationFunctionParams {
  organizationName: string;
  personalAccessTokenValue: string;
}

/**
 * Hook to add a new Azure DevOps organization
 *
 * @returns {UseMutationResult<
 void,
 null,
 IMutationFunctionParams
>} The result of the mutation
 */
const useAddOrganizationMutation = (): UseMutationResult<
  void,
  null,
  IMutationFunctionParams
> => {
  const queryClient = useQueryClient();

  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useMutation({
    mutationFn: async ({
      organizationName,
      personalAccessTokenValue,
    }: IMutationFunctionParams) => {
      try {
        await invoke(COMMAND_ADD_ORGANIZATION, {
          orgaName: organizationName,
          patValue: personalAccessTokenValue,
        });
        updateGlobalNotificationMessage({
          message: "Added DevOps organization",
          type: "Success",
        });
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Error while adding the DevOps organization",
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

export type { IMutationFunctionParams };
export { useAddOrganizationMutation };
