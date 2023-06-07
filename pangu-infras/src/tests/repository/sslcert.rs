use crate::service::DnspodServiceImpl;
use crate::tests::MyAsyncContext;
use pangu_domain::model::{DnsProvider, DnsProviderType};
use pangu_domain::repository::{DnsProviderRepository, Repository};
use pangu_domain::service::sslcert::DnsProviderService;
use test_context::test_context;
#[test_context(MyAsyncContext)]
#[tokio::test]
async fn create_dns_provider(ctx: &mut MyAsyncContext) {
    let dns_provider = DnsProvider {
        id: 0,
        name: "dnspod".to_string(),
        api_key: "123".to_string(),
        api_secret: "test".to_string(),
        provider_type: DnsProviderType::Dnspod,
    };
    let id = ctx.dns_provider_repo.save(dns_provider).await.unwrap();
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
    DnspodServiceImpl::new(DnsProvider::new(
        "na".to_string(),
        "1234".to_string(),
        "12345".to_string(),
        DnsProviderType::Dnspod,
    ))
    .add_record("ab.com", "test", "1.1.1.1", "A")
    .await;
}
