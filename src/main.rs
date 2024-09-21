use std::{collections::HashMap, io::Read, path::PathBuf, str::FromStr};

use select::BlenderVersion;
use serde::Deserialize;

use colored::Colorize;
use tui::TuiApp;

mod getter;
mod select;
mod tracker;
mod tui;

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
    let color_map = HashMap::from([
        ("red", "\x1b[31m"),
        ("green", "\x1b[32m"),
        ("yellow", "\x1b[33m"),
        ("blue", "\x1b[34m"),
        ("magenta", "\x1b[35m"),
        ("cyan", "\x1b[36m"),
    ]);

    let color = |s: &str, color: &str| {
        let color = color_map[color];
        return format!("{}{}{}", color, s, "\x1b[0m");
    };

    fn table_header(is_header: bool) -> String {
        let mut header = String::new();

        let start;
        let end;
        let sep;

        if is_header {
            start = "╭─";
            end = "╮";
            sep = "─┬─";
        } else {
            start = "╰─";
            end = "╯";
            sep = "─┴─";
        }

        for (idx, item) in [10, 9, 18, 15].into_iter().enumerate() {
            if idx == 0 {
                header += start;
            }
            header += &"─".repeat(item);

            if idx == 3 {
                header += end
            } else {
                header += sep
            }
        }

        header
    }

    println!("\nAvailable:\n");

    println!("{}", table_header(true));
    for version in versions.iter() {
        let release = match version.release.as_str() {
            "alpha" => color(&version.release, "magenta"),
            "beta" => color(&version.release, "cyan"),
            "stable" => color(&version.release, "green"),
            "candidate" => color(&version.release, "blue"),
            &_ => "".to_owned(),
        };

        println!(
            "├ {:<10} │ {:<18} │ {:<18} │ {:<15}│",
            version.version, release, version.branch, version.os
        );
    }

    println!("{}", table_header(false));
    println!();
}

fn parse_config() -> anyhow::Result<Config> {
    let path = PathBuf::from_str("config.toml")?;
    if !path.exists() {
        println!("config.toml not found");
        return Ok(Config::default());
    }

    let mut file = std::fs::File::open(path)?;
    let mut buf = Vec::with_capacity(100_000);
    let _ = file.read_to_end(&mut buf).expect("could not read file");

    let contents = String::from_utf8(buf)?;

    let mut config: Config = toml::from_str(&contents).unwrap();

    config.link = "https://builder.blender.org/download/daily/".to_owned();

    config.archive.inspect(|archive| {
        if *archive {
            config.link = "https://builder.blender.org/download/daily/archive/".to_owned();
        }
    });

    Ok(config)
}

fn extract_and_clean(path: PathBuf, config: &Config) {
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

fn download() {
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

            extract_and_clean(path, &config);
        }
    }
}

async fn main_async() {
    let mut app = TuiApp::new();
    let mut terminal = tui::init().unwrap();
    app.run(&mut terminal).await.unwrap();
    tui::restore().unwrap();
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(main_async());
}
