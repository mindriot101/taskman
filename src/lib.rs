#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

pub mod opts;
mod db;
pub mod errors;
mod taskman;
mod priority;

pub use taskman::TaskMan;
