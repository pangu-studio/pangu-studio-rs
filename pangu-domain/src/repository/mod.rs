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
    //save model to db
    async fn save(&mut self, model: M) -> Result<ID, Error>;
    //update model to db
    async fn update(&self, model: M) -> Result<(), Error>;
    //find model by id from db
    async fn find(id: ID) -> Result<M, Error>;
    //remove model
    async fn remove(&mut self, id: ID) -> Result<(), Error>;
}

#[async_trait]
pub trait Delete<ID> {
    //delete model
    async fn delete(id: ID) -> Result<(), Error>;
}
