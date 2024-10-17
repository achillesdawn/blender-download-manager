use std::path::PathBuf;

use crate::{config::Config, BlenderVersion};

pub(super) fn check_downloaded(config: &Config) -> anyhow::Result<Vec<PathBuf>> {
    let path = PathBuf::from(config.path.clone());

    let mut result = Vec::with_capacity(10);

    for dir in std::fs::read_dir(path)? {
        let dir = match dir {
            Ok(d) => d,
            Err(err) => {
                dbg!(err);
                continue;
            }
        };

        let metadata = dir.metadata().unwrap();

        if metadata.is_dir() {
            result.push(dir.path());
        }
    }

    Ok(result)
}

pub(super) fn parse_downloaded(downloaded: Vec<PathBuf>) -> Vec<BlenderVersion> {
    let matcher = crate::blender_utils::BlenderMatcher::new();

    let result: Vec<BlenderVersion> = downloaded
        .into_iter()
        .map(|path| {
            let Some(dir_name) = path.components().last() else {
                return None;
            };

            let dir_name = dir_name.as_os_str().to_str().unwrap();

            matcher.match_str(dir_name)
        })
        .flatten()
        .collect();

    result
}
