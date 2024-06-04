use std::{io::Read, path::PathBuf, str::FromStr};

use select::BlenderVersion;
use serde::Deserialize;

use colored::Colorize;

mod getter;
mod select;
mod tracker;

#[derive(Debug, Deserialize, Default)]
struct Config {
    versions: Vec<String>,
    path: String,
    archive: Option<bool>,
    #[serde(default)]
    link: String,
}

fn check_downloaded(config: &Config) -> anyhow::Result<Vec<PathBuf>> {
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

fn report_available_downloads(versions: &Vec<BlenderVersion>) {
    println!("\nAvailable:\n");
    for version in versions.iter() {
        println!(
            "- {:<10} | {:<10} | {:<15} | {:<15}",
            version.version, version.release, version.branch, version.os
        );
    }
    println!();
}

fn parse_config() -> anyhow::Result<Config> {
    let path = PathBuf::from_str("config.yml")?;
    if !path.exists() {
        println!("config.yml not found");
        return Ok(Config::default());
    }

    let mut file = std::fs::File::open(path)?;
    let mut buf = Vec::with_capacity(100_000);
    let _ = file.read_to_end(&mut buf).expect("could not read file");

    let contents = String::from_utf8(buf)?;

    let mut config: Config = serde_yaml::from_str(&contents)?;

    config.link = "https://builder.blender.org/download/daily/".to_owned();

    config.archive.inspect(|archive| {
        if *archive {
            config.link = "https://builder.blender.org/download/daily/archive/".to_owned();
        }
    });

    Ok(config)
}
fn main() {
    let config = match parse_config() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            Config::default()
        }
    };

    let downloaded = check_downloaded(&config).unwrap();

    let versions = getter::get_links(&config).unwrap();

    report_available_downloads(&versions);

    for version in versions.into_iter() {
        if !config.versions.contains(&version.version) {
            continue;
        }
        dbg!(&version.link);

        let filename = version.link.split("daily/").nth(1).unwrap();

        let mut path = PathBuf::from_str(&config.path).unwrap();
        path.push(filename);

        if downloaded.contains(&path.with_extension("").with_extension("")) {
            println!("{} Already at Latest version", version.version);
            continue;
        }

        if path.exists() {
            println!("{} Already downloaded", version.version);
            continue;
        }

        let mut file = std::fs::File::create(&path).unwrap();

        let download_result = getter::download(&version.link, &mut file);

        if download_result.is_err() {
            println!("Download Error: {}", download_result.err().unwrap());
        } else {
            drop(file);

            println!("{}", "Extracting...".yellow());

            let mut child = std::process::Command::new("tar")
                .arg("-xf")
                .arg(&path)
                .arg(format!("--directory={}", config.path))
                .spawn()
                .unwrap();

            let result = child.wait().unwrap();

            if result.success() {
                println!("{}", "Cleaning up...".yellow());
                std::fs::remove_file(&path).unwrap();
            }

            println!("Downloaded {:?}", path);
        }
    }
}
