use crate::repository::{app_data_path, init_logger};
use crate::service::{DnspodServiceImpl, SSLCertApplicationServiceImpl};
use crate::tests::MyAsyncContext;

use pangu_application::sslcert::{SSLCertApplicationService, SSLCertRequest};
use pangu_domain::model::{DnsProvider, DnsProviderType};
use pangu_domain::repository::{DnsProviderRepository, Repository};
use pangu_domain::service::sslcert::DnsProviderService;
use test_context::test_context;

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn create_dns_provider(ctx: &mut MyAsyncContext) {
    let dns_provider = DnsProvider::new("dnspod", "123", "api_secret", DnsProviderType::Dnspod);
    let id = ctx.dns_provider_repo.create(dns_provider).await.unwrap();
    assert_eq!(id > 0, true)
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn list_endpoints_by_name(ctx: &mut MyAsyncContext) {
    let list = ctx
        .dns_provider_repo
        .find_by_name("dns".to_string())
        .await
        .unwrap();
    println!("dns_provider: {:?}", list);
    assert_eq!(list.len() > 0, true);
}

#[tokio::test]
async fn dnspod_post() {
    let _r = DnspodServiceImpl::new(DnsProvider::new(
        "na",
        "1234",
        "12345",
        DnsProviderType::Dnspod,
    ))
    .add_record("ab.com", "test", "1.1.1.1", "A")
    .await
    .unwrap();
}
#[tokio::test]
async fn remove_record() {
    let _r = DnspodServiceImpl::new(DnsProvider::new(
        "dnspod",
        "232323",
        "12333",
        DnsProviderType::Dnspod,
    ))
    .remove_record("abcd.co", "1495225757")
    .await
    .unwrap();
}

#[tokio::test]
async fn get_cert() -> anyhow::Result<()> {
    app_data_path(".".to_string());
    init_logger(0);
    let dns_provider = DnsProvider::new(
        "dnspod",
        "234603",
        "c57b54a3010a7341e81fed089fbdbd31",
        DnsProviderType::Dnspod,
    );

    let dnspod_svc = Box::new(DnspodServiceImpl::new(dns_provider));

    let ssl_app_svc = SSLCertApplicationServiceImpl::new(dnspod_svc);

    let sslcert_req = SSLCertRequest {
        domain: "pangu.studio".to_string(),
        subdomain: "www".to_string(),
        email: "drmfly.liw@gmail.com".to_string(),
        dns_provider: "dnspod".to_string(),
    };

    ssl_app_svc.create_cert(sslcert_req).await?;
    Ok(())
}
