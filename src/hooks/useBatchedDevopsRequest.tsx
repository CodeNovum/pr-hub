import { Organization } from "../bindings/core";
import { DevOpsRequest } from "../bindings/requests";
import useOrganizations from "./useOrganizations";
import { useRepositories } from "./useRepositories";
import { useMemo } from "react";

/**
 * Hook that retrieves a model used to perform optimized
 * Azure DevOps API requests based on the current state
 *
 * @returns {DevOpsRequest[]} The organizations to include in
 * queries, including their selected repositories
 */
const useBatchedDevopsRequest = (): DevOpsRequest[] => {
  const organizationsQuery = useOrganizations();
  const repositoriesQuery = useRepositories();

  const requestBody = useMemo(
    () =>
      organizationsQuery.data?.map((organization: Organization) => {
        const repoFilter: Array<[string, string] | null> =
          repositoriesQuery.data
            ?.filter(
              (r) =>
                organization?.name &&
                r.project?.organizationName === organization?.name &&
                r.isActive,
            )
            .map((r) => {
              const projectName = r.project?.name ?? "";
              const repoName: string = r.name ?? "";
              return [projectName, repoName];
            }) ?? [];
        const requestModel: DevOpsRequest = {
          repositories: repoFilter,
          organization: organization,
        };
        return requestModel;
      }),
    [organizationsQuery.data, repositoriesQuery.data],
  );

  return requestBody ?? [];
};

export { useBatchedDevopsRequest };
