use crate::{model::Model};
use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointHostType {
    Unix,
    HTTP,
}
#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointType {
    Local,
    Server,
    Edge,
}
// Endpoint
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Display)]
#[display(fmt = "Endpoint: [id={},name={}, host={}", id, name, host)]
pub struct Endpoint {
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub secret: Option<String>,
    pub host_type: EndpointHostType,
    pub endpoint_type: EndpointType,
    pub description: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
}
impl Model for Endpoint {
    fn table_name() -> String {
        return "endpoints".to_string();
    }
}
impl Endpoint {
    pub fn new(
        name: &str,
        host: &str,
        port: i32,
        secret: Option<&str>,
        host_type: EndpointHostType,
        endpoint_type: EndpointType,
    ) -> Self {
        let dt = Utc::now();
        Endpoint {
            id: 0,
            name: name.to_string(),
            host: host.to_string(),
            port: port,
            secret: Some(secret.unwrap_or("").to_string()),
            host_type: host_type,
            endpoint_type: endpoint_type,
            description: None,
            create_time: Some(dt),
            deleted: None,
        }
    }
    pub fn edit_desc(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }
}
