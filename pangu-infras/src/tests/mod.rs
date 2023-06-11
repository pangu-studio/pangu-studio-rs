// mod store;
mod repository;
mod service;

use std::fs;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

use once_cell::sync::OnceCell as SyncCell;
use pangu_application::sslcert::SSLCertApplicationService;
use pangu_domain::errors::Error;
use pangu_domain::repository::SSLCertificateRepository;
use pangu_domain::service::sslcert::DnsProviderService;

use crate::repository::{
    DnsProviderRepositoryImpl, EndpointRepositoryImpl, SSLCertificateRepositoryImpl,
};
use crate::service::{DnspodServiceImpl, SSLCertApplicationServiceImpl};
use crate::{app_data_path, db_conn_pool, init_logger, run_migrations};

use test_context::{test_context, AsyncTestContext, TestContext};

///tokio runtime for sync testing
pub fn runtime() -> Result<&'static Runtime, Error> {
    static RUNTIME: SyncCell<Runtime> = SyncCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| Err(Error::Runtime(err.to_string()))))
}

struct Services {
    dns_provider_svc: Box<dyn DnsProviderService + Send + Sync>,
}
struct AppServices {
    sslcert_app_svc: Box<dyn SSLCertApplicationService + Send + Sync>,
}
struct Repositories {
    endpoint_repo: EndpointRepositoryImpl,
    dns_provider_repo: DnsProviderRepositoryImpl,
    ssl_cert_repo: Box<dyn SSLCertificateRepository + Send + Sync>,
}
pub struct MyAsyncContext {
    // value: String,
    repositories: Repositories,
    services: Services,
    app_services: AppServices,
}

pub struct MyContext {
    value: String,
}

#[async_trait::async_trait]
impl AsyncTestContext for MyAsyncContext {
    async fn setup() -> MyAsyncContext {
        initialize().await;
        let db_pool = db_conn_pool().await.unwrap();
        let dns_provider_repo = DnsProviderRepositoryImpl::new(db_pool);
        let dnspod_svc = DnspodServiceImpl::new(Box::new(dns_provider_repo.clone()));
        let ssl_cert_repo = Box::new(SSLCertificateRepositoryImpl::new(db_pool));

        MyAsyncContext {
            // value: "test".to_string()
            repositories: Repositories {
                endpoint_repo: EndpointRepositoryImpl::new(db_pool),
                dns_provider_repo: dns_provider_repo.clone(),
                ssl_cert_repo: ssl_cert_repo.clone(),
            },
            services: Services {
                dns_provider_svc: Box::new(DnspodServiceImpl::new(Box::new(
                    dns_provider_repo.clone(),
                ))),
            },
            app_services: AppServices {
                sslcert_app_svc: Box::new(SSLCertApplicationServiceImpl::new(
                    Box::new(dnspod_svc),
                    Box::new(dns_provider_repo.clone()),
                    ssl_cert_repo.clone(),
                )),
            },
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
