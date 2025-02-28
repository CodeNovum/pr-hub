import {
  COMMAND_UPDATE_PAT,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
} from "../constants";
import {
  UseMutationResult,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "react-toastify";

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

  return useMutation({
    mutationFn: async (mutationVars: IMutationParams) => {
      try {
        await invoke(COMMAND_UPDATE_PAT, {
          id: mutationVars.id,
          pat: mutationVars.pat,
        });
        toast("Updated the Personal Access Token", { type: "success" });
      } catch (error) {
        console.error(error);
        toast("Could not update the Personal Access Token", { type: "error" });
      }
    },
    onSettled: () =>
      queryClient.invalidateQueries({
        queryKey: [RQ_KEY_IMPORTED_GIT_REPOSITORIES],
      }),
  });
};

export { useUpdatePatMutation };
