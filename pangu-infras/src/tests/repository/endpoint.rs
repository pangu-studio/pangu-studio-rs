use crate::{repository, tests::MyAsyncContext};
use test_context::test_context;

use pangu_domain::repository::EndpointRepository;

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn list_endpoints(_ctx: &mut MyAsyncContext) {
    let er = repository::EndpointRepositoryImpl::new();

    let rs = er.list_endpoints().await.unwrap();

    for r in rs.iter() {
        println!("endpoint: {:?}", r);
    }
    assert_eq!(rs.len() > 0, true)
}
