use crate::errors::Error;
use crate::model::{DnsProvider, SSLCertificate};
use crate::repository::Repository;
use async_trait::async_trait;
#[async_trait]
pub trait DnsProviderRepository: Repository<DnsProvider, i64> {
    async fn find_by_name(&self, name: String) -> Result<Vec<DnsProvider>, Error>;
    async fn list_dns_providers(&self) -> Result<Vec<DnsProvider>, Error>;
}
#[async_trait]
pub trait SSLCertificateRepository: Repository<SSLCertificate, i64> {
    async fn find_by_domain(&self, domains: &str) -> Result<Vec<SSLCertificate>, Error>;
}
