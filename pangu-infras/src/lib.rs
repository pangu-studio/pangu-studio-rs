#[macro_use]
extern crate log;
extern crate simplelog;
extern crate serde_derive;

pub mod service;
pub mod repository;
mod init;
pub use init::*;

#[cfg(test)]
mod tests;

