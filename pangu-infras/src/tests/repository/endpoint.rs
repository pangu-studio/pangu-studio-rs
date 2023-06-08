use crate::{repository, tests::MyAsyncContext};
use test_context::test_context;

use pangu_domain::model::{Endpoint, EndpointHostType, EndpointType};
use pangu_domain::repository::{EndpointRepository, Repository};

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn list_endpoints(ctx: &mut MyAsyncContext) {
    let rs = ctx.endpoint_repo.list_endpoints().await.unwrap();

    for r in rs.iter() {
        println!("endpoint: {:?}", r);
    }
    assert_eq!(rs.len() > 0, true)
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn create_endpoint(ctx: &mut MyAsyncContext) {
    let endpoint = Endpoint::new(
        "test",
        "localhost",
        2354,
        Some("123"),
        EndpointHostType::HTTP,
        EndpointType::Server,
    );
    let id = ctx.endpoint_repo.create(endpoint).await.unwrap();
    println!("endpoint id: {:?}", id);
}
