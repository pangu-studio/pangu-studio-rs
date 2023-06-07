use crate::errors::Error;
use crate::model::DnsProvider;
use crate::repository::Repository;
use async_trait::async_trait;
#[async_trait]
pub trait DnsProviderRepository: Repository<DnsProvider, i64> {
    async fn find_by_name(&self, name: String) -> Result<Vec<DnsProvider>, Error>;
}
