use anyhow::Result;
use application::traits::SecretRepository;
use keyring::Entry;

/// Repository to access secrets using keyring
pub struct KeyringRepository {
    service_name: String,
}

impl KeyringRepository {
    /// Creates a new instance
    ///
    /// # Arguments
    ///
    /// * `service_name` - The service identifier to use for storing secrets
    pub fn new(service_name: String) -> Self {
        Self { service_name }
    }
}

impl SecretRepository for KeyringRepository {
    fn get_secret(&self, key: &str) -> Result<String> {
        Ok(Entry::new(&self.service_name, key)?.get_password()?)
    }

    fn set_secret(&self, key: &str, value: &str) -> Result<()> {
        Ok(Entry::new(&self.service_name, key)?.set_password(value)?)
    }

    fn delete_secret(&self, key: &str) -> Result<()> {
        Ok(Entry::new(&self.service_name, key)?.delete_password()?)
    }
}
