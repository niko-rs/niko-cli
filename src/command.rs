use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "niko", about = "command line tool for the niko game engine")]
pub enum Command {
    New {
        #[structopt(name = "NAME", help = "the name of the project")]
        name: String,

        #[structopt(name = "AUTHOR", short = "a", long = "author", help = "the author put into Cargo.toml")]
        author: Option<String>,

        #[structopt(name = "PATH", short = "p", long = "path", help = "the path to create the project in")]
        path: Option<PathBuf>,
    },
    Watch,
    Bundle,
}

impl Command {
    pub fn parse() -> Self {
        Command::from_args()
    }
}
