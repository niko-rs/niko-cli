use std::process::Command;
use crate::Error;

pub fn run_wasm_pack() -> Result<(), Error> {
    let mut child = Command::new("wasm-pack")
        .args(&["build", "--target", "web"])
        .spawn()?;
    child.wait()?;
    Ok(())
}
