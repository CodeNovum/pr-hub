use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectionData {
    #[serde(rename = "authenticatedUser")]
    authenticated_user: AuthedUser,
    #[serde(rename = "authorizedUser")]
    authorized_user: AuthedUser,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "deploymentId")]
    deployment_id: String,
    #[serde(rename = "deploymentType")]
    deployment_type: String,
    #[serde(rename = "locationServiceData")]
    location_service_data: LocationServiceData,
}

#[derive(Serialize, Deserialize)]
pub struct AuthedUser {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "descriptor")]
    descriptor: String,
    #[serde(rename = "subjectDescriptor")]
    subject_descriptor: String,
    #[serde(rename = "providerDisplayName")]
    provider_display_name: String,
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(rename = "properties")]
    properties: Properties,
    #[serde(rename = "resourceVersion")]
    resource_version: i64,
    #[serde(rename = "metaTypeId")]
    meta_type_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "Account")]
    account: Account,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "$type")]
    account_type: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocationServiceData {
    #[serde(rename = "serviceOwner")]
    service_owner: String,
    #[serde(rename = "defaultAccessMappingMoniker")]
    default_access_mapping_moniker: String,
    #[serde(rename = "lastChangeId")]
    last_change_id: i64,
    #[serde(rename = "lastChangeId64")]
    last_change_id64: i64,
}
