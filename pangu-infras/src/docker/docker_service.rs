use bollard::container::{InspectContainerOptions, ListContainersOptions};
use bollard::{ClientVersion, Docker};
use pangu_domain::service::container_service::ContainerService;
use std::collections::HashMap;
use std::default::Default;

pub struct DockerService {
    // pub docker_client: DockerClient,
}
use pangu_domain::model::ContainerSummary;
async fn conc(arg: (Docker, &ContainerSummary)) {
    let (docker, container) = arg;
    println!(
        "{:?}",
        docker
            .inspect_container(
                container.id.as_ref().unwrap(),
                None::<InspectContainerOptions>
            )
            .await
            .unwrap()
    )
}
use async_trait::async_trait;

#[async_trait]
impl ContainerService for DockerService {
    async fn list_containers(&self) -> Vec<ContainerSummary> {
        let docker = Docker::connect_with_http(
            "http://localhost:2345",
            120,
            &ClientVersion {
                major_version: 1,
                minor_version: 41,
            },
            Some("46af768f12bd195ab".to_string()),
        ).unwrap();

        let v = docker.clone().negotiate_version().await;
        println!("{:?}", v);
        let mut list_container_filters = HashMap::new();
        list_container_filters.insert("status", vec!["running"]);

        let containers = docker
            .list_containers(Some(ListContainersOptions {
                all: true,
                filters: list_container_filters,
                ..Default::default()
            }))
            .await;
        containers.unwrap()
    }
}
