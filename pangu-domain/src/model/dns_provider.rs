use crate::model::Model;
use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum DnsProviderType {
    Dnspod,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Display)]
#[display(
    fmt = "Endpoint: [id={},name={}, api_key={}, api_secret=***,provider_type={}]",
    id,
    name,
    api_key,
    provider_type
)]
pub struct DnsProvider {
    pub id: i64,
    pub name: String,
    pub api_key: String,
    pub api_secret: String,
    pub provider_type: DnsProviderType,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}
impl DnsProvider {
    pub fn new(
        name: &str,
        api_key: &str,
        api_secret: &str,
        provider_type: DnsProviderType,
    ) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            provider_type,
            create_time: Some(Utc::now()),
            update_time: None,
        }
    }
}
impl Model for DnsProvider {
    fn table_name() -> String {
        return "dns_providers".to_string();
    }
}

// ssl_cert
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Display)]
#[display(
    fmt = "Endpoint: [id={}, domains={}, cert_chain={}, private_key={},deleted={:?}], create_time={:?}, update_time={:?}",
    id,
    domains,
    cert_chain,
    private_key,
    deleted,
    create_time,
    update_time
)]
pub struct SSLCertificate {
    pub id: i64,
    pub domains: String,
    pub cert_chain: String,
    pub private_key: String,
    pub deleted: Option<bool>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}
impl SSLCertificate {
    pub fn new(domains: &str, cert_chain: &str, private_key: &str) -> Self {
        Self {
            id: 0,
            domains: domains.to_string(),
            cert_chain: cert_chain.to_string(),
            private_key: private_key.to_string(),
            create_time: Some(Utc::now()),
            update_time: None,
            deleted: Some(false),
        }
    }
}
impl Model for SSLCertificate {
    fn table_name() -> String {
        return "ssl_certificates".to_string();
    }
}
