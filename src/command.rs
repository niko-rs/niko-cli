use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "niko", about = "command line tool for the niko game engine")]
pub enum Command {
    #[structopt(about = "Creates a new project as a sub-directory with the given <name>")]
    New {
        #[structopt(name = "NAME", help = "the name of the project")]
        name: String,

        #[structopt(name = "AUTHOR", short = "a", long = "author", help = "the author put into Cargo.toml")]
        author: Option<String>,

        #[structopt(name = "PATH", short = "p", long = "path", help = "the path to create the project in")]
        path: Option<PathBuf>,
    },
    #[structopt(about = "Creates a new project in the current directory and uses the directory name as the project name")]
    Init {
        #[structopt(name = "AUTHOR", short = "a", long = "author", help = "the author put into Cargo.toml")]
        author: Option<String>,
    },
    #[structopt(about = "Runs the development server and opens the project in the system's configured browser")]
    Watch,
    #[structopt(about = "Bundles the project as a release that you can upload to itch.io")]
    Bundle,
}

impl Command {
    pub fn parse() -> Self {
        Command::from_args()
    }
}
