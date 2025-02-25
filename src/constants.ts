const COMMAND_IMPORT_AZURE_DEVOPS_ORGANIZATION =
  "import_azure_devops_organization_repositories";
const COMMAND_GET_REPOSITORIES = "get_git_repositories";
const COMMAND_REMOVE_REPOSITORY = "remove_git_repository";
const COMMAND_TOGGLE_REPOSITORY_ACTIVE = "toggle_git_repository_active_state";
const COMMAND_UPDATE_PAT = "update_pat_for_git_repository";

const RQ_KEY_IMPORTED_GIT_REPOSITORIES = "imported-repositories";

export {
  COMMAND_GET_REPOSITORIES,
  COMMAND_IMPORT_AZURE_DEVOPS_ORGANIZATION,
  COMMAND_TOGGLE_REPOSITORY_ACTIVE,
  COMMAND_REMOVE_REPOSITORY,
  COMMAND_UPDATE_PAT,
  RQ_KEY_IMPORTED_GIT_REPOSITORIES,
};
