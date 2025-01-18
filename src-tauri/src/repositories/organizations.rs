use crate::data::application_database::get_db_connection;
use anyhow::Result;
use rusqlite::params;

/// Retrieve the list of all organizations from the database.
///
/// # Returns
///
/// The list of all organizations tuples, containing the database id and the name.
///
pub fn get_organizations() -> Vec<(i32, String)> {
    let db_connection = get_db_connection();
    let mut stmt = db_connection
        .prepare("SELECT id, name FROM organizations")
        .unwrap();
    let orga_iter = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap();
    let mut organizations = Vec::new();
    for orga in orga_iter {
        match orga {
            Ok(orga) => organizations.push(orga),
            Err(err) => println!("err: {:?}", err),
        }
    }
    organizations
}

/// Add an organization name to the database.
///
/// # Arguments
///
/// * `orga_name` - The name of the organization to add.
///
pub fn add_organization(orga_name: &str) -> Result<i64> {
    let db_connection = get_db_connection();
    let mut smtm = db_connection
        .prepare("INSERT INTO organizations (name) VALUES (?1)")
        .unwrap();
    let id = smtm.insert(params!(orga_name.to_string()));
    Ok(id?)
}

/// Remove an organization from the database.
///
/// # Arguments
///
/// * `id` - The id of the organization to remove.
///
/// # Returns
///
/// The number of rows removed.
///
pub fn remove_organization(id: &i64) -> Result<usize> {
    let db_connection = get_db_connection();
    let mut smtm = db_connection
        .prepare("DELETE FROM organizations WHERE id = ?1")
        .unwrap();
    let result = smtm.execute(params!(id));
    Ok(result?)
}
