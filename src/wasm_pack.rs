use std::process::Command;

pub fn run_wasm_pack() {
    let mut child = Command::new("wasm-pack")
        .args(&["build", "--target", "web"])
        .spawn()
        .expect("could not run wasm-pack");
    child.wait().expect("could not wait on child output");
}
