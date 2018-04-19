extern crate structopt;
extern crate taskman;

use taskman::TaskMan;
use taskman::opts::Opts;
use structopt::StructOpt;

fn main() {
    let opt = Opts::from_args();
    TaskMan::from_opts(opt).expect("error").run();
}
