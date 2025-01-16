use crate::{
    constants::SERVICE_NAME,
    model::core::organization::Organization,
    repositories::{self, devops, organizations},
};
use anyhow::Result;
use keyring::Entry;

/// Retrieve the list of all organizations the user has stored.
///
/// # Arguments
///
/// * `include_pat` - Whether to include the PAT of the organizations in the result or not.
///
/// # Returns
///
/// The list of all organizations the user has stored.
///
pub async fn get_organizations(include_pat: bool) -> Result<Vec<Organization>> {
    let stored_organization_names = organizations::get_organizations();
    let mut organizations = Vec::new();
    for orga_tuple in stored_organization_names {
        let id = orga_tuple.0;
        let name = orga_tuple.1;
        let is_pat_valid: bool;
        let mut pat_to_use = "".to_string();
        let stored_pat = Entry::new(SERVICE_NAME, &format!("organization-{}", id));
        match stored_pat {
            Ok(pat) => {
                let pat = pat.get_password();
                match pat {
                    Ok(p) => {
                        if include_pat {
                            pat_to_use = p.to_string();
                        }
                        is_pat_valid = devops::validate_pat(&p, &name).await;
                    }
                    Err(_) => is_pat_valid = false,
                }
            }
            Err(_) => is_pat_valid = false,
        }
        organizations.push(Organization {
            name,
            id,
            is_pat_valid,
            pat: pat_to_use,
        });
    }
    Ok(organizations)
}

/// Add an organization.
///
/// # Arguments
///
/// * `orga_name` - The name of the organization to add.
///
pub fn add_organization(orga_name: &str, pat_value: &str) -> Result<i64> {
    let id = repositories::organizations::add_organization(orga_name)?;
    let secret = Entry::new(SERVICE_NAME, &format!("organization-{}", id))?;
    _ = secret.set_password(pat_value);
    Ok(id)
}

/// Remove an organization.
///
/// # Arguments
///
/// * `id` - The id of the organization to remove.
///
pub fn remove_organization(id: &i64) -> Result<()> {
    let secret = Entry::new(SERVICE_NAME, &format!("organization-{}", id))?;
    _ = secret.delete_password();
    let result = repositories::organizations::remove_organization(id);
    match result {
        Ok(_) => {}
        Err(err) => return Err(err),
    }
    Ok(())
}

/// Updates the stored PAT for an organization.
///
/// # Arguments
///
/// * `id` - The id of the organization to update.
/// * `pat_value` - The new PAT value to store.
///
pub fn update_pat(id: &i64, pat_value: &str) -> Result<()> {
    let secret = Entry::new(SERVICE_NAME, &format!("organization-{}", id))?;
    _ = secret.set_password(pat_value);
    Ok(())
}
