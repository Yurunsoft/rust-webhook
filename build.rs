use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("manifest_dir={}", manifest_dir);
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("out_dir={}", out_dir);
    let output_path = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
    println!("output_path={}", output_path.to_str().unwrap().to_string());

    println!("Copy config.json...");
    fs::copy(Path::new(&manifest_dir).join("config.json"), output_path.join("config.json")).unwrap();
}
