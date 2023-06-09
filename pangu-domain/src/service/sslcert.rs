use crate::errors::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Status {
    code: String,
    message: String,
    created_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Record {
    pub id: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub weight: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseData {
    pub status: Status,
    pub record: Option<Record>,
}
#[async_trait]
pub trait DnsProviderService {
    async fn add_record(
        &self,
        provider_id: i64,
        domain: &str,
        subdomain: &str,
        value: &str,
        record_type: &str,
    ) -> Result<ResponseData, Error>;
    async fn remove_record(
        &self,
        provider_id: i64,
        domain: &str,
        record_id: &str,
    ) -> Result<(), Error>;
}
