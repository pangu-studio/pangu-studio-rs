pub mod docker;
pub mod service;

use pangu_domain::service::container_service::ContainerService;
use pangu_domain::service::sslcert::DnsProvider;

use crate::service::sslcert_dnspod::DnspodService;
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
    #[test]
    fn list() {
        let ds = DnspodService::new("a", "b");

        ds.add_record("a".to_string(), "a".to_string(), "a".to_string());
    }
}

#[tokio::test]
async fn list_containers() {
    let ds = docker::docker_service::DockerService {};
    let list = ds.list_containers().await;
    println!("{:?}", list);
}
