//! Builds scss files with sass to produce css.
//! Currently the sass is only built on non-windows platforms.
//! Makes new data folder if not already there.

use dotenv::dotenv;

use std::{
    env,
    fs::{self, File},
    io::prelude::*,
    process::Command,
};

fn compile_sass() {
    dotenv().ok();
    if !cfg!(target_os = "windows") && !env::var("STOP_CARGO_BUILD_SASS").is_ok() {
        let output = Command::new("sass")
            .arg("style:style")
            .status()
            .expect("failed to execute sass process");
        if !output.success() {
            panic!("Failed to compile css")
        }
        println!("{:#?}", output)
    };
}

fn make_data_folder() {
    fs::create_dir_all("data/guest/projects").unwrap();
    let mut file = File::create("data/guest/user_info.json").unwrap();
    file.write("{\"projects\": []}".as_bytes()).unwrap();
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn main() {
    compile_sass();
    if !path_exists("data") {
        make_data_folder()
    }
}
