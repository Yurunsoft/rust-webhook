use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_path = Path::new(&manifest_dir_string).join("target").join(env::var("PROFILE").unwrap());

    println!("Copy config.json...");
    fs::copy(Path::new(&manifest_dir_string).join("config.json"), Path::new(&output_path).join("config.json")).unwrap();
}
