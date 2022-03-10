use patch::Patch;
use std::process::Command;

fn main() {
    generate_patch();
}

fn generate_patch() {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .unwrap();
    println!("{:?}", output);
}
