mod container_credential;
mod endpoint; 
mod dns_provider;
pub use container_credential::*;
pub use endpoint::*;
pub use dns_provider::*;


pub trait Model {
    fn table_name()->String;
}