use std::ops::Add;

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

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum SSLCertStatus {
    Pending,
    Success,
    Invalid,
}
// ssl_cert
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Display)]
#[display(
    fmt = "Endpoint: [id={}, sn={}, domains={}, cert_chain={}, private_key={}, status = {},deleted={:?}], create_time={:?}, update_time={:?}",
    id,
    sn,
    domains,
    cert_chain,
    private_key,
    status,
    deleted,
    create_time,
    update_time
)]
pub struct SSLCertificate {
    pub id: i64,
    pub sn: String,
    pub domains: String,
    pub mail: String,
    pub cert_chain: String,
    pub private_key: String,
    pub status: SSLCertStatus,
    pub deleted: Option<bool>,
    pub addition: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
    pub expire_time: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Display, Default)]
#[display(
    fmt = "SSLCertificateAddition: [identifier={}, record_value={}, record_type={}]",
    identifier,
    record_value,
    record_type
)]
pub struct SSLCertificateAddition {
    pub identifier: String,
    pub record_value: String,
    pub record_type: String,
}
impl SSLCertificate {
    pub fn new(
        sn: &str,
        mail: &str,
        domains: &str,
        cert_chain: &str,
        private_key: &str,
        addition: &str,
    ) -> Self {
        Self {
            id: 0,
            sn: sn.to_string(),
            mail: mail.to_string(),
            domains: domains.to_string(),
            cert_chain: cert_chain.to_string(),
            private_key: private_key.to_string(),
            status: SSLCertStatus::Pending,
            addition: Some(addition.to_string()),
            create_time: Some(Utc::now()),
            update_time: None,
            expire_time: None,
            deleted: Some(false),
        }
    }
    //签发证书
    pub fn issue(&mut self, cert_chain: &str, private_key: &str) {
        self.cert_chain = cert_chain.to_string();
        self.private_key = private_key.to_string();
        self.status = SSLCertStatus::Success;
        self.update_time = Some(Utc::now());
        self.expire_time = Some(Utc::now().add(chrono::Duration::days(90)));
    }
}
impl Model for SSLCertificate {
    fn table_name() -> String {
        return "ssl_certificates".to_string();
    }
}
