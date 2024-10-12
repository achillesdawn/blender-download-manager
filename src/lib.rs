pub mod config;
mod getter;
pub mod blender_utils;
// mod tracker;
pub mod tui;

#[derive(Debug)]
pub struct BlenderVersion {
    pub version: String,
    pub release: String,
    pub branch: String,
    pub os: String,
    pub link: String,
}
