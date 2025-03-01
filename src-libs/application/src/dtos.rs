use chrono::{DateTime, Utc};
use domain::{
    enums::GitProvider as DomainGitProvider,
    models::{GitRepository, PullRequest},
};
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Clone, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub enum GitProvider {
    AzureDevOps,
}

impl From<DomainGitProvider> for GitProvider {
    fn from(value: DomainGitProvider) -> Self {
        match value {
            DomainGitProvider::AzureDevOps => GitProvider::AzureDevOps,
        }
    }
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct GitRepositoryDto {
    pub id: u32,
    pub name: String,
    pub context: String,
    #[ts(rename = "gitProvider")]
    #[serde(rename = "gitProvider")]
    pub git_provider: GitProvider,
    #[ts(rename = "isActive")]
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl From<&GitRepository> for GitRepositoryDto {
    fn from(value: &GitRepository) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
            context: value.context.to_string(),
            git_provider: value.git_provider.clone().into(),
            is_active: value.is_active,
        }
    }
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct PullRequestDto {
    #[ts(rename = "repositoryName")]
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    pub title: String,
    #[ts(rename = "mergeStatus")]
    #[serde(rename = "mergeStatus")]
    pub merge_status: String,
    #[ts(rename = "creatorName")]
    #[serde(rename = "creatorName")]
    pub creator_name: String,
    #[ts(type = "Date | string | null")]
    #[ts(rename = "creationDate")]
    #[serde(rename = "creationDate")]
    pub creation_date: DateTime<Utc>,
    #[ts(rename = "numberOfComments")]
    #[serde(rename = "numberOfComments")]
    pub number_of_comments: usize,
    #[ts(rename = "numberOfClosedComments")]
    #[serde(rename = "numberOfClosedComments")]
    pub number_of_closed_comments: usize,
}

impl From<&PullRequest> for PullRequestDto {
    fn from(value: &PullRequest) -> Self {
        Self {
            repository_name: value.repository_name.to_string(),
            title: value.title.to_string(),
            merge_status: value.merge_status.to_string(),
            creator_name: value.creator_name.to_string(),
            creation_date: value.creation_date,
            number_of_closed_comments: value.number_of_closed_comments,
            number_of_comments: value.number_of_comments,
        }
    }
}
