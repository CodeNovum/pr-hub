import {
  COMMAND_IMPORT_AZURE_DEVOPS_ORGANIZATION,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
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
const useAddAzureDevOpsOrganizationMutation = (): UseMutationResult<
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
        await invoke(COMMAND_IMPORT_AZURE_DEVOPS_ORGANIZATION, {
          organizationName: organizationName,
          pat: personalAccessTokenValue,
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
      queryClient.invalidateQueries({
        queryKey: [RQ_KEY_IMPORTED_GIT_REPOSITORIES],
      });
    },
  });
};

export type { IMutationFunctionParams };
export { useAddAzureDevOpsOrganizationMutation };
