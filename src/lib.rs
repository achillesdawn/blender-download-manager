use std::path::PathBuf;
use config::Config;

pub mod blender_utils;
pub mod config;
mod getter;
mod tracker;
// mod tracker;
pub mod tui;

#[derive(Debug, Clone)]
pub struct BlenderVersion {
    pub version: String,
    pub release: String,
    pub branch: String,
    pub os: String,
    pub link: String,
}

pub fn extract_and_clean(path: PathBuf, config: &Config) {
    println!("{}", "Extracting...");

    let mut child = std::process::Command::new("tar")
        .arg("-xf")
        .arg(&path)
        .arg(format!("--directory={}", config.path))
        .spawn()
        .unwrap();

    let result = child.wait().unwrap();

    if result.success() {
        println!("{}", "Cleaning up...");
        std::fs::remove_file(&path).unwrap();
    }

    println!("Downloaded {:?}", path);
}
