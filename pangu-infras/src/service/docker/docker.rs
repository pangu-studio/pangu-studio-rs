use async_trait::async_trait;
use bollard::container::{InspectContainerOptions, ListContainersOptions};
use bollard::models::ContainerSummary;
use bollard::Docker;
use pangu_application::container::ContainerService;
use pangu_application::container::ListContainerRequest;
use pangu_domain::errors::Error;
use std::default::Default;

pub struct ContainerServiceImpl {
    docker_client: Docker,
}

impl ContainerServiceImpl {
    pub fn new() -> Self {
        ContainerServiceImpl {
            docker_client: Docker::connect_with_http_defaults().unwrap(),
        }
    }
    pub fn get_instance(client: Docker) -> impl ContainerService {
        ContainerServiceImpl {
            docker_client: client,
        }
    }
}

async fn _conc(arg: (Docker, &ContainerSummary)) {
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
#[async_trait]
impl ContainerService for ContainerServiceImpl {
    async fn list_containers(
        &self,
        req: ListContainerRequest,
    ) -> Result<Vec<ContainerSummary>, Error> {
        self.docker_client
            .clone()
            .negotiate_version()
            .await
            .unwrap();

        let containers = self
            .docker_client
            .list_containers(Some(ListContainersOptions {
                all: true,
                filters: req.filters,
                ..Default::default()
            }))
            .await
            .or_else(|err| Err(Error::Docker(err.to_string())))?;
        Ok(containers)
    }
}
