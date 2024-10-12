pub mod config;
mod getter;
mod select;
mod tracker;
pub mod tui;

#[derive(Debug)]
pub struct BlenderVersion {
    pub version: String,
    pub release: String,
    pub branch: String,
    pub os: String,
    pub link: String,
}
