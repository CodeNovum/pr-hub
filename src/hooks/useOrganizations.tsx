import { Organization } from "../bindings/core";
import { COMMAND_GET_ORGANIZATIONS } from "../constants";
import { useStoreActions } from "../store/store";
import { UseQueryResult, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

/**
 * Hook to retrieve all imported Azure DevOps organizations
 *
 * @returns {UseQueryResult<Organization[]>} The query result
 */
const useOrganizations = (): UseQueryResult<Organization[]> => {
  const updateGlobalNotificationMessage = useStoreActions(
    (actions) => actions.ApplicationModel.updateGlobalNotificationMessage,
  );

  return useQuery({
    queryKey: ["user-organizations"],
    queryFn: async () => {
      try {
        const result = await invoke<Organization[]>(COMMAND_GET_ORGANIZATIONS);
        return result;
      } catch (error) {
        console.error(error);
        updateGlobalNotificationMessage({
          message: "Could not retrieve organizations",
          type: "Error",
        });
        return [];
      }
    },
  });
};

export default useOrganizations;
