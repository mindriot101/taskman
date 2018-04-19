use priority::Priority;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "taskman", about = "Task manager")]
pub enum Opts {
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "d", long = "description")]
        description: String,
        #[structopt(short = "t", long = "tag")]
        tag: Vec<String>,
        #[structopt(short = "p", long = "priority")]
        priority: Option<Priority>,
    },
}
