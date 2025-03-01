use super::enums::GitProvider;
use chrono::{DateTime, Utc};

/// Represents a single git repository
#[derive(Debug)]
pub struct GitRepository {
    /// The unique identifier
    pub id: u32,
    /// The repository name
    pub name: String,
    /// The context in that the git repository can be accessed on
    /// the remote. This is vendor specific and varies between
    /// different git providers.
    pub context: String,
    /// The git provider where the repository is stored
    pub git_provider: GitProvider,
    /// When marked as active, the repository will be included when
    /// querying data to fulfill the applications purpose
    pub is_active: bool,
    /// The key to retrieve the PAT to use for this git repository
    /// from the secret storage
    pub pat_secret_key: String,
}

/// Represents a single pull request within a git repository
pub struct PullRequest {
    /// The unique identifier
    pub id: u32,
    /// The name of the associated git repository
    pub repository_name: String,
    /// The pull request title
    pub title: String,
    /// The label that describes the merge status
    pub merge_status: String,
    /// The display name of the user that created the pull request
    pub creator_name: String,
    /// The date of the pull request creation
    pub creation_date: DateTime<Utc>,
    /// The total number of reviewers comments
    pub number_of_comments: usize,
    /// The total number of closed comments
    /// (closed, resolved, etc.)
    pub number_of_closed_comments: usize,
}
