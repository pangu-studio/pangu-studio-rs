use crate::model::ContainerSummary;
use async_trait::async_trait;
#[async_trait]
pub trait ContainerService  {
    async fn list_containers(&self) -> Vec<ContainerSummary>;

}