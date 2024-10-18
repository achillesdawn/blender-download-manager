use std::{path::PathBuf, time::Duration};

use crate::{config::Config, LocalBlenderVersion};

pub(super) fn check_downloaded(config: &Config) -> Result<Vec<(PathBuf, Duration)>, String> {
    let path = PathBuf::from(config.path.clone());

    let mut result = Vec::with_capacity(10);

    for dir in std::fs::read_dir(path).unwrap() {
        let dir = dir.unwrap();

        let metadata = dir.metadata().unwrap();

        if metadata.is_dir() {
            let elapsed = metadata.created().unwrap().elapsed().unwrap();
            result.push((dir.path(), elapsed));
        }
    }

    Ok(result)
}

fn duration_to_human_readable(duration: Duration) -> String {
    let total_secs = duration.as_secs();

    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    const DAY: u64 = 24 * HOUR;
    const WEEK: u64 = 7 * DAY;
    const MONTH: u64 = 30 * DAY;
    const YEAR: u64 = 365 * DAY;

    match total_secs {
        0..=59 => "just now".to_string(),

        secs if secs < HOUR => {
            let mins = secs / MINUTE;
            format!(
                "{} {} ago",
                mins,
                if mins == 1 { "minute" } else { "minutes" }
            )
        }
        secs if secs < DAY => {
            let hours = secs / HOUR;
            format!(
                "{} {} ago",
                hours,
                if hours == 1 { "hour" } else { "hours" }
            )
        }
        secs if secs < WEEK => {
            let days = secs / DAY;
            format!("{} {} ago", days, if days == 1 { "day" } else { "days" })
        }
        secs if secs < MONTH => {
            let weeks = secs / WEEK;
            format!(
                "{} {} ago",
                weeks,
                if weeks == 1 { "week" } else { "weeks" }
            )
        }
        secs if secs < YEAR => {
            let months = secs / MONTH;
            format!(
                "{} {} ago",
                months,
                if months == 1 { "month" } else { "months" }
            )
        }
        _ => {
            let years = total_secs / YEAR;
            format!(
                "{} {} ago",
                years,
                if years == 1 { "year" } else { "years" }
            )
        }
    }
}

pub(super) fn parse_downloaded(downloaded: Vec<(PathBuf, Duration)>) -> Vec<LocalBlenderVersion> {
    let matcher = crate::blender_utils::BlenderMatcher::new();

    let result: Vec<LocalBlenderVersion> = downloaded
        .into_iter()
        .map(|(path, duration)| {
            let Some(dir_name) = path.components().last() else {
                return None;
            };

            let dir_name = dir_name.as_os_str().to_str().unwrap();

            if let Some(version) = matcher.match_str(dir_name) {
                return Some(LocalBlenderVersion {
                    blender_version: version,
                    created: duration_to_human_readable(duration),
                });
            }
            None
        })
        .flatten()
        .collect();

    result
}

