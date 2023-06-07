use crate::service::docker::{docker_client, ContainerServiceImpl};
use pangu_application::container::{ContainerService, ListContainerRequest};
use std::collections::HashMap;
#[tokio::test]
pub async fn list_containers() {
    let cli = docker_client(
        "http://localhost:2345",
        Some("46af768f12bd195a".to_string()),
    );

    let cs = ContainerServiceImpl::get_instance(cli.lock().unwrap().clone());

    let mut list_container_filters = HashMap::new();
    list_container_filters.insert("status".to_string(), vec!["running".to_string(),"exited".to_string()]);
    list_container_filters.insert("name".to_string(), vec!["redis".to_string()]);
    let req = ListContainerRequest {
        all: true,
        limit: None,
        size: false,
        filters: list_container_filters,
    };

    let rs = cs.list_containers(req).await;

    for r in rs.iter() {
        println!("container: {:?}", r);
    }
}
