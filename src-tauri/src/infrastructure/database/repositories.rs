use super::daos::GitRepositoryDao;
use crate::{application::traits::GitRepositoryRepository, domain::models::GitRepository};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

/// Repository to access the git repositories, stored in the database
pub struct GitRepositoryDatabaseRepository {
    database_access: Arc<SqlitePool>,
}

impl GitRepositoryDatabaseRepository {
    /// Create a new instance
    ///
    /// # Arguments
    ///
    /// * `database_access` - Access to the target database
    pub fn new(database_access: Arc<SqlitePool>) -> Self {
        Self { database_access }
    }
}

#[async_trait]
impl GitRepositoryRepository for GitRepositoryDatabaseRepository {
    async fn get_all_git_repositories(&self) -> Result<Vec<GitRepository>> {
        let git_repositories = sqlx::query_as::<_, GitRepositoryDao>(
            r#"
                    SELECT *
                    FROM git_repositories
            "#,
        )
        .fetch_all(&*self.database_access)
        .await?;
        let result = git_repositories.iter().map(|x| x.into()).collect();
        Ok(result)
    }

    async fn get_git_repository_by_id(&self, id: &u32) -> Result<GitRepository> {
        let git_repo = sqlx::query_as::<_, GitRepositoryDao>(
            r#"
                SELECT *
                FROM git_repositories
                WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.database_access)
        .await?;
        let result = git_repo.into();
        Ok(result)
    }

    async fn update_git_repository(&self, git_repository: GitRepository) -> Result<()> {
        let dao: GitRepositoryDao = git_repository.into();
        sqlx::query(
            r#"
                UPDATE git_repositories
                SET name = ?1, context = ?2, is_active = ?3, git_provider = ?4
                WHERE id = ?5;
            "#,
        )
        .bind(dao.name)
        .bind(dao.context)
        .bind(dao.is_active)
        .bind(dao.git_provider)
        .bind(dao.id)
        .execute(&*self.database_access)
        .await?;
        Ok(())
    }

    async fn create_git_repository(&self, git_repository: GitRepository) -> Result<()> {
        let dao: GitRepositoryDao = git_repository.into();
        sqlx::query(
            r#"
                INSERT INTO git_repositories (name, context, is_active, git_provider, pat_secret_key)
                VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(dao.name)
        .bind(dao.context)
        .bind(dao.is_active)
        .bind(dao.git_provider)
        .bind(dao.pat_secret_key)
        .execute(&*self.database_access)
        .await?;
        Ok(())
    }

    async fn delete_git_repository(&self, id: &u32) -> Result<()> {
        sqlx::query(
            r#"
                DELETE FROM git_repositories
                WHERE id = ?1
            "#,
        )
        .bind(id)
        .execute(&*self.database_access)
        .await?;
        Ok(())
    }
}
