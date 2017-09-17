#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

pub mod error;
pub use error::Error;

pub mod cli;
pub mod command;
