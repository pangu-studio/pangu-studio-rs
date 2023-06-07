#[macro_use]
extern crate log;
extern crate simplelog;
extern crate serde_derive;

pub mod docker;
pub mod service;
pub mod repository;

#[cfg(test)]
mod tests;

