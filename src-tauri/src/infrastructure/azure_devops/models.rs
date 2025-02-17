use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::enums::{CommentType, PullRequestCommentThreadStatus};

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-request-threads/list?view=azure-devops-rest-6.0&tabs=HTTP#comment
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Comment {
    #[serde(rename = "commentType")]
    pub comment_type: CommentType,
}

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-requests/get-pull-requests?view=azure-devops-rest-6.0&tabs=HTTP#identityref
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct IdentityRef {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub id: String,
}

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-requests/get-pull-requests?view=azure-devops-rest-6.0&tabs=HTTP#gitrepository
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct GitRepository {
    pub id: String,
    pub name: String,
}

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-requests/get-pull-requests?view=azure-devops-rest-6.0&tabs=HTTP#webapitagdefinition
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct WebApiTagDefinition {
    pub id: String,
    pub name: String,
}

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/core/projects/list?view=azure-devops-rest-6.0&tabs=HTTP#teamprojectreference
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct TeamProjectReference {
    pub name: String,
}

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-requests/get-pull-requests?view=azure-devops-rest-6.0&tabs=HTTP#gitpullrequest
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct PullRequest {
    #[serde(rename = "createdBy")]
    pub created_by: IdentityRef,
    #[serde(rename = "creationDate")]
    pub creation_date: DateTime<Utc>,
    pub labels: Vec<WebApiTagDefinition>,
    #[serde(rename = "mergeStatus")]
    pub merge_status: String,
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: i32,
    pub repository: GitRepository,
    pub reviewers: Vec<IdentityRef>,
    pub status: String,
    pub title: String,
}

/// https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-request-threads/list?view=azure-devops-rest-6.0&tabs=HTTP#gitpullrequestcommentthread
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct PullRequestCommentThread {
    pub comments: Vec<Comment>,
    pub status: PullRequestCommentThreadStatus,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Response<T> {
    pub value: Vec<T>,
}
