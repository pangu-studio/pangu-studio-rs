use async_trait::async_trait;
use pangu_domain::errors::Error;
use pangu_domain::{
    model::{DnsProvider, Model},
    repository::{DnsProviderRepository, Repository},
};
use sqlx::{Pool, Sqlite};
/// ==================================================================
/// ====================DnsProviderRepositoryImpl=====================
/// ==================================================================
#[derive(Debug, Clone)]
pub struct DnsProviderRepositoryImpl {
    db_pool: &'static Pool<Sqlite>,
}

impl DnsProviderRepositoryImpl {
    pub fn new(db_pool: &'static Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl Repository<DnsProvider, i64> for DnsProviderRepositoryImpl {
    async fn create(&self, provider: DnsProvider) -> Result<i64, Error> {
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
    async fn update(&self, provider: DnsProvider) -> Result<(), Error> {
        let sql = format!(
            r#"
            UPDATE {} SET name = ?1, api_key = ?2, api_secret = ?3, provider_type = ?4, update_time = ?5 WHERE id = ?6;
            "#,
            DnsProvider::table_name()
        );
        sqlx::query(&sql)
            .bind(provider.name)
            .bind(provider.api_key)
            .bind(provider.api_secret)
            .bind(provider.provider_type)
            .bind(provider.update_time)
            .bind(provider.id)
            .execute(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
    async fn find(&self, id: i64) -> Result<DnsProvider, Error> {
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE id = ?1;
            "#,
            DnsProvider::table_name()
        );
        let provider = sqlx::query_as::<Sqlite, DnsProvider>(&sql)
            .bind(id)
            .fetch_one(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(provider)
    }
    async fn remove(&self, id: i64) -> Result<(), Error> {
        let sql = format!(
            r#"
            UPDATE {} SET delete = ?1 WHERE id = ?2;
            "#,
            DnsProvider::table_name()
        );
        sqlx::query(&sql)
            .bind(true)
            .bind(id)
            .execute(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
}
/// ==================================================================
/// ====================DnsProviderRepositoryImpl=====================
/// ==================================================================
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

    async fn list_dns_providers(&self) -> Result<Vec<DnsProvider>, Error> {
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE deleted = ?1;
            "#,
            DnsProvider::table_name()
        );
        let list = sqlx::query_as::<Sqlite, DnsProvider>(&sql)
            .bind(false)
            .fetch_all(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(list)
    }
}