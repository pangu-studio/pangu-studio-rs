mod container_credential;
mod endpoint; 
pub use container_credential::*;
pub use endpoint::*;


pub trait Model {
    fn table_name()->String;
}