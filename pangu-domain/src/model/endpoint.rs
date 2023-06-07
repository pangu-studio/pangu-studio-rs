use crate::model::Model;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

use serde::{Serialize, Deserialize};

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
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
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
    fn table_name()->String {
        return "endpoints".to_string();
    }
}