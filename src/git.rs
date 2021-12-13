use crate::{common, ARGS};

pub fn init() {
    let current_dir = std::env::current_dir().expect("should get current dir");
    std::env::set_current_dir(ARGS.root_path()).expect("package directory should exist");
    std::process::Command::new("git")
        .arg("init")
        .output()
        .expect("should initialize git repository");
    common::write_file(".gitignore", "target/");
    std::env::set_current_dir(current_dir)
        .expect("should be able to go back to previous directory");
}
