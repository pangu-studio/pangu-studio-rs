use crate::errors::Error;
use crate::model::Model;
use async_trait::async_trait;
mod endpoint;
mod sslcert;
pub use endpoint::*;
pub use sslcert::*;
//Model trait define
#[async_trait]
pub trait Repository<M: Model, ID> {
    //create model to db
    async fn create(&self, model: M) -> Result<ID, Error>;
    //update model to db
    async fn update(&self, model: M) -> Result<(), Error>;
    //find model by id from db
    async fn find(&self, id: ID) -> Result<M, Error>;
    //remove model
    async fn remove(&self, id: ID) -> Result<(), Error>;
}

#[async_trait]
pub trait Delete<ID> {
    //delete model
    async fn delete(&self, id: ID) -> Result<(), Error>;
}
