use std::collections::HashMap;

use pangu_bollard::models::ContainerSummary;
use async_trait::async_trait;
use pangu_domain::errors::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListContainerRequest {
    pub all: bool,
    pub limit: Option<i64>,
    pub size: bool,
    pub filters: HashMap<String,Vec<String>>,
}
#[async_trait]
pub trait ContainerService {
    async fn list_containers(&self,req:ListContainerRequest) -> Result<Vec<ContainerSummary>, Error>;
}
