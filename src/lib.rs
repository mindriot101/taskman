extern crate diesel;
extern crate dotenv;
extern crate failure;
#[macro_use]
extern crate structopt;

pub mod opts;
mod db;
pub mod errors;
mod taskman;

pub use taskman::TaskMan;
