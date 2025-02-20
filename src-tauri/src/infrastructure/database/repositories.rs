use crate::application::traits::GitRepositoryRepository;

/// Repository to access the git repositories, stored in the database
pub struct GitRepositoryDatabaseRepository {}

impl GitRepositoryRepository for GitRepositoryDatabaseRepository {}
