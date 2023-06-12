use async_trait::async_trait;
use pangu_domain::errors::Error;
use pangu_domain::repository::Delete;
use pangu_domain::{
    model::{Model, SSLCertificate},
    repository::{Repository, SSLCertificateRepository},
};
use sqlx::{Pool, Sqlite};
/// ======================================================================
/// =====================SSLCertificateRepositoryImpl=====================
/// ======================================================================
#[derive(Debug, Clone)]
pub struct SSLCertificateRepositoryImpl {
    db_pool: &'static Pool<Sqlite>,
}

impl SSLCertificateRepositoryImpl {
    pub fn new(db_pool: &'static Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}
#[async_trait]
impl Repository<SSLCertificate, i64> for SSLCertificateRepositoryImpl {
    async fn create(&self, cert: SSLCertificate) -> Result<i64, Error> {
        let sql = format!(
            r#"
        INSERT INTO {} (domains, sn, mail, cert_chain, private_key, deleted, addition, create_time) VALUES (?1,?2,?3,?4,?5,?6,?7, ?8);
        "#,
            SSLCertificate::table_name()
        );

        let id = sqlx::query(&sql)
            .bind(cert.domains)
            .bind(cert.sn)
            .bind(cert.mail)
            .bind(cert.cert_chain)
            .bind(cert.private_key)
            .bind(cert.deleted)
            .bind(cert.addition)
            .bind(cert.create_time)
            .execute(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?
            .last_insert_rowid();
        Ok(id)
    }
    async fn update(&self, _cert: SSLCertificate) -> Result<(), Error> {
        let sql = format!(
            r#"
            UPDATE {} SET domains = ?1, cert_chain = ?2, private_key = ?3, status = ?4, deleted = ?5, update_time = ?6 WHERE id = ?7;
            "#,
            SSLCertificate::table_name()
        );
        sqlx::query(&sql)
            .bind(_cert.domains)
            .bind(_cert.cert_chain)
            .bind(_cert.private_key)
            .bind(_cert.status)
            .bind(_cert.deleted)
            .bind(_cert.update_time)
            .bind(_cert.id)
            .execute(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
    async fn find(&self, id: i64) -> Result<SSLCertificate, Error> {
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE id = ?1;
            "#,
            SSLCertificate::table_name()
        );
        let cert = sqlx::query_as::<Sqlite, SSLCertificate>(&sql)
            .bind(id)
            .fetch_one(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(cert)
    }
    async fn remove(&self, id: i64) -> Result<(), Error> {
        let sql = format!(
            r#"
            UPDATE {} SET deleted = ?1 WHERE id = ?2;
            "#,
            SSLCertificate::table_name()
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
#[async_trait]
impl SSLCertificateRepository for SSLCertificateRepositoryImpl {
    async fn find_by_domain(&self, domains: &str) -> Result<Vec<SSLCertificate>, Error> {
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE domains LIKE ?1;
            "#,
            SSLCertificate::table_name()
        );
        let list = sqlx::query_as::<Sqlite, SSLCertificate>(&sql)
            .bind(format!("%{}%", domains))
            .fetch_all(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(list)
    }
    async fn find_all(&self) -> Result<Vec<SSLCertificate>, Error> {
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE deleted = ?1;
            "#,
            SSLCertificate::table_name()
        );
        let list = sqlx::query_as::<Sqlite, SSLCertificate>(&sql)
            .bind(false)
            .fetch_all(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(list)
    }
    async fn find_by_sn(&self, sn: &str) -> Result<SSLCertificate, Error> {
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE sn = ?1;
            "#,
            SSLCertificate::table_name()
        );
        let cert = sqlx::query_as::<Sqlite, SSLCertificate>(&sql)
            .bind(sn)
            .fetch_one(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(cert)
    }
}
#[async_trait]
impl Delete<i64> for SSLCertificateRepositoryImpl {
    async fn delete(&self, id: i64) -> Result<(), Error> {
        let sql = format!(
            r#"
            DELETE FROM {} WHERE id = ?1;
            "#,
            SSLCertificate::table_name()
        );
        sqlx::query(&sql)
            .bind(id)
            .execute(self.db_pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
}
