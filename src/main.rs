mod command;
mod projects;
mod wasm_pack;
mod server;

use command::Command;

#[tokio::main]
pub async fn main() {
    match Command::parse() {
        Command::New { name, author, path, } => {

            let path = if let Some(path) = path {
                path
            } else if let Some(path) = std::env::current_dir().ok() {
                path
            } else {
                panic!("path was not specified and could not be inferred");
            };

            let author = if let Some(author) = author {
                author
            } else if let Some(author) = projects::find_author_name() {
                author
            } else {
                panic!("author was not specified and could not be inferred");
            };

            // creates a new project
            projects::generate_project(path, name, author);

        },
        Command::Watch => {
            // watches for changes and recompiles

            wasm_pack::run_wasm_pack();
            server::run_server().await;
        },
        Command::Bundle => {
            // builds the current project
        },
    }
}
