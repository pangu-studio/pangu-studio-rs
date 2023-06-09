use crate::tests::MyAsyncContext;

use pangu_application::sslcert::SSLCertRequest;
use pangu_domain::model::{DnsProvider, DnsProviderType};
use pangu_domain::repository::{DnsProviderRepository, Repository};
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
    let res = ctx.dns_provider_repo.find_by_name("dns".to_string()).await;
    assert!(res.is_ok());
}
#[test_context(MyAsyncContext)]
#[tokio::test]
async fn dnspod_post(ctx: &mut MyAsyncContext) {
    let _r = ctx
        .services
        .dns_provider_svc
        .add_record(1, "ab.com", "test", "1.1.1.1", "A")
        .await
        .unwrap();
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn remove_record(ctx: &mut MyAsyncContext) {
    let _r = ctx
        .services
        .dns_provider_svc
        .remove_record(1, "abcd.co", "1495225757")
        .await
        .unwrap();
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn get_cert(ctx: &mut MyAsyncContext) -> anyhow::Result<()> {
    // let ssl_app_svc = SSLCertApplicationServiceImpl::new(dnspod_svc);

    let sslcert_req = SSLCertRequest {
        provider_id: 1,
        domain: "test.studio".to_string(),
        subdomain: "www".to_string(),
        email: "a.b@gmail.com".to_string(),
        dns_provider: "dnspod".to_string(),
    };

    ctx.app_services
        .sslcert_app_svc
        .create_cert(sslcert_req)
        .await?;
    Ok(())
}
