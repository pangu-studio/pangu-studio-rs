use crate::model::container::Container;
pub trait ContainerService  {
    fn list_containers(&self) -> Vec<Container>;
}