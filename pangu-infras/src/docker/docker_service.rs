use pangu_domain::service::container_service::ContainerService;

pub struct DockerService {
    // pub docker_client: DockerClient,
}
impl ContainerService for DockerService {
    fn list_containers(&self) -> Vec<pangu_domain::model::container::Container> {
        todo!()
    }
}
