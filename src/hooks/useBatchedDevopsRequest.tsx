import { Organization } from "../bindings/core";
import { DevOpsRequest } from "../bindings/requests";
import { useDevOpsProjects } from "./useDevOpsProjects";
import useOrganizations from "./useOrganizations";
import { useMemo } from "react";

/**
 * Hook that retrieves a model used to perform optimized
 * Azure DevOps API requests based on the current state
 *
 * @returns {DevOpsRequest[]} The organizations to include in
 * queries, including their selected projects
 */
const useBatchedDevopsRequest = (): DevOpsRequest[] => {
  const organizationsQuery = useOrganizations();
  const projectsQuery = useDevOpsProjects();

  const requestBody = useMemo(
    () =>
      organizationsQuery.data?.map((organization: Organization) => {
        const organizationProjectNames =
          projectsQuery.data
            ?.filter(
              (p) =>
                organization?.name &&
                p.organizationName === organization?.name &&
                p.isActive,
            )
            .map((p) => p.name) ?? [];
        const requestModel: DevOpsRequest = {
          projectNames: organizationProjectNames as string[],
          organization: organization,
        };
        return requestModel;
      }),
    [organizationsQuery.data, projectsQuery.data],
  );

  return requestBody ?? [];
};

export { useBatchedDevopsRequest };
