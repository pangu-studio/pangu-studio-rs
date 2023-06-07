mod docker;
pub use docker::*;
use once_cell::sync::OnceCell;
use bollard::{Docker,ClientVersion};
use std::sync::Mutex;


static DOCKER_CLIENT_TIMEOUT: u64 = 120;
static CLIENT: OnceCell<Mutex<Docker>> = OnceCell::new();

pub fn docker_client(addr: &str,secret:Option<String>) -> &'static Mutex<Docker> {
    CLIENT.get_or_init(|| {
            let docker = Docker::connect_with_http(
                addr,
                DOCKER_CLIENT_TIMEOUT,
                &ClientVersion {
                    major_version: 1,
                    minor_version: 41,
                },
                Some(secret.unwrap_or("".to_string())),
            ).unwrap();
    
        Mutex::new(docker)
    }
    )
}