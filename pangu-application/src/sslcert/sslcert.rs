use async_trait::async_trait;
use derive_more::Display;
use pangu_domain::{
    errors::Error,
    model::{DnsProvider, SSLCertificate},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display(
    fmt = "SSLCertRequest: [domain={},mail={}, dns_provider={}",
    domain,
    mail,
    provider_id
)]
pub struct SSLCertRequest {
    pub provider_id: i64,
    pub mail: String,
    pub domain: String,
    pub subdomain: String,
    pub sn: String,
    // pub dns_provider: String,
}
#[async_trait]
pub trait SSLCertApplicationService {
    async fn create_cert(&self, cert: SSLCertRequest) -> Result<(), Error>;
    async fn list_dns_providers(&self) -> Result<Vec<DnsProvider>, Error>;
    async fn list_sslcerts(&self) -> Result<Vec<SSLCertificate>, Error>;
    async fn get_sslcert_by_sn(&self, sn: &str) -> Result<SSLCertificate, Error>;
}
