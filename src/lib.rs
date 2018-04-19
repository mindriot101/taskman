#[macro_use]
extern crate structopt;

pub mod opts;

pub struct TaskMan {}

impl TaskMan {
    pub fn from_opts(opts: opts::Opts) -> Self {
        TaskMan {}
    }

    pub fn run(&mut self) {}
}
