use crate::application::traits::AzureDevOpsRepository;

/// Responsible for importing all git repositories from a single
/// Azure DevOps organization
pub struct DevOpsOrgaImporter {
    azure_devops_repository: Box<dyn AzureDevOpsRepository>,
}

impl DevOpsOrgaImporter {
    /// Create a new instance of the importer
    ///
    /// # Arguments
    ///
    /// * `azure_devops_repository` - The repository to get the git repositories
    pub fn new(azure_devops_repository: Box<dyn AzureDevOpsRepository>) -> Self {
        Self {
            azure_devops_repository,
        }
    }

    /// Import all the git repositories by querying the from the Azure
    /// DevOps organization, store them in the database and store the
    /// PAT in the secret storage
    ///
    /// # Arguments
    ///
    /// * `organization_name` - The name of the Azure DevOps organization
    /// * `pat` - The PAT to access all git repositories
    pub async fn import(&self, organization_name: &str, pat: &str) {
        let _git_repositories = self
            .azure_devops_repository
            .get_repositories_in_organization(pat, organization_name)
            .await;
        println!("{:?}", _git_repositories);
        // TODO: Store the repos in the database
        // TODO: Store the pat in the secret store
        println!("Hello");
    }
}
