//! Builds scss files with sass to produce css.
//! Currently the sass is only built on non-windows platforms.

use dotenv::dotenv;
use std::env;
use std::process::Command;

fn main() {
    dotenv().ok();
    if !cfg!(target_os = "windows")
        && !env::var("STOP_CARGO_BUILD_SASS").is_ok()
    {
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
