use chrono::{DateTime, Utc};

/// Represents a single git repository
#[derive(Debug)]
pub struct GitRepository {
    /// The repository name
    pub name: String,
    /// The context in that the git repository can be accessed on
    /// the remote. This is vendor specific and varies between
    /// different git providers.
    pub context: String,
    /// When marked as active, the repository will be included when
    /// querying data to fulfill the applications purpose
    pub is_active: bool,
}

/// Represents a single pull request within a git repository
pub struct PullRequest {
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
    pub number_of_comments: u16,
    /// The total number of closed comments
    /// (closed, resolved, etc.)
    pub number_of_closed_comments: u16,
}
