use async_trait::async_trait;
use pangu_domain::repository::{Repository,EndpointRepository};
use pangu_domain::model::{Endpoint,EndpointHostType,EndpointType, Model};
use pangu_domain::errors::Error;

use super::{db, db_conn_pool};
pub struct EndpointRepositoryImpl {

}

impl EndpointRepositoryImpl {
    pub fn new() -> Self {
        EndpointRepositoryImpl{}
    }
}

#[async_trait]
impl Repository<Endpoint,i64> for EndpointRepositoryImpl {
        //save model to db
        async fn save(&mut self) -> Result<i64, Error> {
            return Ok(0);
        }
        //update model to db
        async fn update(&self) -> Result<(), Error> {
            return Ok(());
        }
        //find model by id from db
        async fn find(id: i64) -> Result<Endpoint, Error> {
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
        async fn remove(&mut self) -> Result<(), Error> {
            return Ok(());
        }
}
#[async_trait]
impl EndpointRepository<Endpoint,i64> for EndpointRepositoryImpl {
    async fn list_endpoints(&self) -> Result<Vec<Endpoint>, Error> {
        let pool = db_conn_pool().await?;
        let sql = format!("select * from {}",Endpoint::table_name());
        let rows = sqlx::query_as::<_,Endpoint>(&sql)
        .fetch_all(pool)
        .await
        .or_else(|err| Err(Error::Database(err)))?;
        Ok(rows)
    }
}
    