use std::process::Command;

fn main() {
    if !cfg!(target_os = "windows") {
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
