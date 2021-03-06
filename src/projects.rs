use tinytemplate::{TinyTemplate, format_unescaped};
use std::path::{PathBuf, Path};
use serde_derive::Serialize;
use crate::Error;

#[derive(Serialize)]
struct Context {
    name: String,
    author: String,
}

pub fn generate_project(root_dir: PathBuf, name: String, author: String) -> Result<(), Error> {
    use std::fs;
    use git2::{Repository, RepositoryInitOptions};
    
    let mut options = RepositoryInitOptions::new();
    options.initial_head("main");
    Repository::init_opts(&root_dir, &options)?;

    let mut src_path = root_dir.clone();
    src_path.push("src");
    fs::create_dir_all(&src_path)?;

    let context = Context {
        name,
        author,
    };

    let mut file_path = root_dir.clone();
    file_path.push("Cargo.toml");
    render_template_to_file(&file_path, include_str!("../templates/Cargo.toml.template"), &context)?;

    let mut file_path = root_dir.clone();
    file_path.push(".gitignore");
    copy_template_to_file(&file_path, include_str!("../templates/.gitignore.template"))?;

    let mut file_path = root_dir.clone();
    file_path.push("src");
    file_path.push("lib.rs");
    copy_template_to_file(&file_path, include_str!("../templates/src/lib.rs.template"))?;
    

    let mut file_path = root_dir.clone();
    file_path.push("index.html");
    copy_template_to_file(&file_path, include_str!("../templates/index.html.template"))?;
    

    let mut file_path = root_dir.clone();
    file_path.push("index.js");
    render_template_to_file(&file_path, include_str!("../templates/index.js.template"), &context)?;

    Ok(())
}

fn render_template_to_file(file: &Path, template: &str, context: &Context) -> Result<(), Error> {
    use std::fs;
    use std::io::prelude::*;

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("template", template)?;

    let rendered = tt.render("template", context)?;

    let mut file = fs::File::create(file)?;
    file.write_all(rendered.as_bytes())?;
    Ok(())
}

fn copy_template_to_file(file: &Path, template: &str) -> Result<(), Error> {
    use std::fs;
    use std::io::prelude::*;

    let mut file = fs::File::create(file)?;
    file.write_all(template.as_bytes())?;
    Ok(())
}

pub fn find_author_name() -> Option<String> {
    use git2::Config;

    let config = Config::open_default().ok()?;
    let name = config.get_string("user.name").ok()?;
    let email = config.get_string("user.email").ok()?;

    Some(format!("{} <{}>", name, email))
}
