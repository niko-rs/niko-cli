mod command;
mod projects;
mod wasm_pack;
mod server;

use command::Command;

#[tokio::main]
pub async fn main() {
    match Command::parse() {
        Command::New { name, author, path, } => {

            let mut path = if let Some(path) = path {
                path
            } else if let Some(path) = std::env::current_dir().ok() {
                path
            } else {
                panic!("path was not specified and could not be inferred");
            };

            // add the name on top of the path
            path.push(&name);

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
        Command::Init { author, } => {
            let path = if let Some(path) = std::env::current_dir().ok() {
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

            let name = if let Some(name) = path.file_name() {
                name.to_string_lossy().to_string()
            } else {
                panic!("name was not inferred from current location");
            };

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
