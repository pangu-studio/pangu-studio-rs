use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::model::Model;

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum DnsProviderType {
    Dnspod,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Display)]
#[display(fmt = "Endpoint: [id={},name={}, api_key={}, api_secret=***,provider_type={}]", id, name, api_key,provider_type)]
pub struct DnsProvider {
    pub id: i64,
    pub name: String,
    pub api_key: String,
    pub api_secret: String,
    pub provider_type: DnsProviderType,
}
impl DnsProvider {
    pub fn new(name: String, api_key: String, api_secret: String, provider_type: DnsProviderType) -> Self {
        Self {
            id: 0,
            name,
            api_key,
            api_secret,
            provider_type,
        }
    }
}
impl Model for DnsProvider {
    fn table_name() -> String {
        return "dns_providers".to_string();
    }
}