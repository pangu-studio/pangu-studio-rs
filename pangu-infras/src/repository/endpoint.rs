use async_trait::async_trait;
use chrono::Utc;
use pangu_domain::errors::Error;
use pangu_domain::model::{Endpoint, EndpointHostType, EndpointType, Model};
use pangu_domain::repository::{EndpointRepository, Repository};

use super::{db, db_conn_pool};
pub struct EndpointRepositoryImpl {}

impl EndpointRepositoryImpl {
    pub fn new() -> Self {
        EndpointRepositoryImpl {}
    }
}

#[async_trait]
impl Repository<Endpoint, i64> for EndpointRepositoryImpl {
    //save model to db
    async fn save(&mut self, mut endpoint: Endpoint) -> Result<i64, Error> {
        let pool = db_conn_pool().await?;
        let sql = format!("insert into {} (name,host,port,secret,host_type,endpoint_type,create_time) values (?1,?2,?3,?4,?5,?6,?7)",Endpoint::table_name());
        debug!("sql:{}\nendpoint:{}", sql, endpoint);
        let mut builder = sqlx::query(&sql)
            .bind(endpoint.name)
            .bind(endpoint.host)
            .bind(endpoint.port)
            .bind(endpoint.secret)
            .bind(endpoint.host_type)
            .bind(endpoint.endpoint_type);

        if endpoint.create_time.is_none() {
            let dt = Utc::now();
            endpoint.create_time = Some(dt);
        }
        builder = builder.bind(endpoint.create_time);

        let id = builder
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?
            .last_insert_rowid();
        endpoint.id = id;
        Ok(id)
    }
    //update model to db
    async fn update(&self, endpoint: Endpoint) -> Result<(), Error> {
        return Ok(());
    }
    //find model by id from db
    async fn find(_id: i64) -> Result<Endpoint, Error> {
        todo!("not implement");
        //     return Ok(Endpoint{
        //         host: "".to_string(),
        //         port: "".to_string(),
        //         secret: None,
        //         endpoint_type: EndpointType::Local,
        //         host_type: EndpointHostType::Unix,
        //     });
    }
    //remove model
    async fn remove(&mut self, id: i64) -> Result<(), Error> {
        todo!("not implement");
        return Ok(());
    }
}
#[async_trait]
impl EndpointRepository<Endpoint, i64> for EndpointRepositoryImpl {
    async fn list_endpoints(&self) -> Result<Vec<Endpoint>, Error> {
        let pool = db_conn_pool().await?;
        let sql = format!("select * from {}", Endpoint::table_name());
        let rows = sqlx::query_as::<_, Endpoint>(&sql)
            .fetch_all(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(rows)
    }
}
