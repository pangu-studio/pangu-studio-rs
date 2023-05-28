pub mod docker;

use pangu_domain::service::container_service::ContainerService;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[tokio::test]
async fn list_containers() {
    let ds = docker::docker_service::DockerService {};
    let list = ds.list_containers().await;
    println!("{:?}", list);
}
