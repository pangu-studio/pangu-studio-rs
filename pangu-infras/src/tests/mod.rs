// mod store;
mod repository;
mod service;

use once_cell::sync::OnceCell as SyncCell;
use pangu_domain::errors::Error;
use std::fs;

use tokio::sync::OnceCell;

use crate::repository::db_conn_pool;
use crate::repository::run_migrations;
use crate::repository::DnsProviderRepositoryImpl;
use crate::repository::{app_data_path, init_logger, EndpointRepositoryImpl};
// use crate::store::project::{save_project, delete_project, list_projects, Project};

use test_context::{test_context, AsyncTestContext, TestContext};

use tokio::runtime::Runtime;

///tokio runtime for sync testing
pub fn runtime() -> Result<&'static Runtime, Error> {
    static RUNTIME: SyncCell<Runtime> = SyncCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| Err(Error::Runtime(err.to_string()))))
}

pub struct MyAsyncContext {
    // value: String,
    endpoint_repo: EndpointRepositoryImpl,
    dns_provider_repo: DnsProviderRepositoryImpl,
}

pub struct MyContext {
    value: String,
}

#[async_trait::async_trait]
impl AsyncTestContext for MyAsyncContext {
    async fn setup() -> MyAsyncContext {
        initialize().await;
        MyAsyncContext {
            // value: "test".to_string()
            endpoint_repo: EndpointRepositoryImpl::new(),
            dns_provider_repo: DnsProviderRepositoryImpl::new(),
        }
    }

    async fn teardown(self) {
        // Perform any teardown you wish.
    }
}

impl TestContext for MyContext {
    fn setup() -> MyContext {
        let rt = runtime().unwrap();

        rt.block_on(initialize());
        // block_on()
        MyContext {
            value: "test".to_string(),
        }
    }

    fn teardown(self) {
        // Perform any teardown you wish.
    }
}

static ONCE: OnceCell<anyhow::Result<()>> = OnceCell::const_new();

pub async fn initialize() -> &'static anyhow::Result<()> {
    ONCE.get_or_init(|| async {
        let test_folder = ".".to_string();
        app_data_path(test_folder.to_string());
        init_logger(0);

        fs::remove_file("./data.db").unwrap_or_else(|why| error!("! {:?}", why.kind()));
        match run_migrations().await {
            Ok(_) => {
                info!("migrations done");
            }
            Err(e) => {
                error!("migrations failed: {}", e);
            }
        };

        //read sql file
        let sql = match fs::read_to_string("./db/test/data.sql") {
            Ok(sql) => sql,
            Err(_) => {
                //找不到测试数据sql文件直接退出
                panic!("test data sql file not found")
            }
        };
        debug!("sql file {}", sql);
        //insert test data
        sqlx::query(sql.as_str())
            .execute(db_conn_pool().await?)
            .await?;
        Ok(())
    })
    .await
}

#[test_context(MyContext)]
#[test]
fn test_works(ctx: &mut MyContext) {
    assert_eq!(ctx.value, "test")
}
