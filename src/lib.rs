pub mod blender_utils;
pub mod config;
mod getter;
mod tracker;
pub mod tui;

#[derive(Debug, Clone)]
pub struct BlenderVersion {
    pub version: String,
    pub release: String,
    pub branch: String,
    pub os: String,
    pub link: String,
}

pub struct LocalBlenderVersion {
    pub blender_version: BlenderVersion,
    pub created: String,
}
