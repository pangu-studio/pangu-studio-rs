use crate::model::Endpoint;
use crate::repository::Repository;
use crate::errors::Error;

use async_trait::async_trait;
#[async_trait]
pub trait EndpointRepository<Model, ID>: Repository<Endpoint, i64> {
    async fn list_endpoints(&self) -> Result<Vec<Endpoint>, Error>;
}
