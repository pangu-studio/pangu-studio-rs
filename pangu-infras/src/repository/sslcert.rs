use crate::repository::db_conn_pool;
use async_trait::async_trait;
use futures::executor::block_on;
use pangu_domain::errors::Error;
use pangu_domain::{
    model::{DnsProvider, Model},
    repository::{DnsProviderRepository, Repository},
};
use sqlx::{Pool, Sqlite};
pub struct DnsProviderRepositoryImpl {
    db_pool: &'static Pool<Sqlite>,
}

impl DnsProviderRepositoryImpl {
    pub fn new() -> Self {
        Self {
            db_pool: block_on(async { db_conn_pool().await.unwrap() }),
        }
    }
}

#[async_trait]
impl Repository<DnsProvider, i64> for DnsProviderRepositoryImpl {
    async fn save(&mut self, provider: DnsProvider) -> Result<i64, Error> {
        let sql = format!(
            r#"
        INSERT INTO {} (name, api_key, api_secret, provider_type) VALUES (?1,?2,?3,?4)
        "#,
            DnsProvider::table_name()
        );

        let id = sqlx::query(&sql)
            .bind(provider.name)
            .bind(provider.api_key)
            .bind(provider.api_secret)
            .bind(provider.provider_type)
            .execute(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?
            .last_insert_rowid();
        Ok(id)
    }
    async fn update(&self, model: DnsProvider) -> Result<(), Error> {
        unimplemented!()
    }
    async fn find(id: i64) -> Result<DnsProvider, Error> {
        unimplemented!()
    }
    async fn remove(&mut self, id: i64) -> Result<(), Error> {
        unimplemented!()
    }
}
#[async_trait]
impl DnsProviderRepository for DnsProviderRepositoryImpl {
    async fn find_by_name(&self, name: String) -> Result<Vec<DnsProvider>, Error> {
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE name LIKE ?1;
            "#,
            DnsProvider::table_name()
        );
        // .bind(format!("%{}%","dn"))
        let list = sqlx::query_as::<Sqlite, DnsProvider>(&sql)
            .bind(format!("%{}%", name))
            .fetch_all(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(list)
    }
}
