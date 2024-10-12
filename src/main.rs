use std::path::PathBuf;

use regex::Regex;
use select::BlenderVersion;

use tui::TuiApp;

mod config;
mod getter;
mod select;
mod tracker;
mod tui;

use config::{parse_config, Config};

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

fn extract_and_clean(path: PathBuf, config: &Config) {
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

// fn download() {
//     let config = match parse_config() {
//         Ok(config) => config,
//         Err(err) => {
//             println!("{}", err);
//             Config::default()
//         }
//     };

//     let downloaded = check_downloaded(&config).unwrap();

//     let versions = getter::get_links(&config).unwrap();

//     for version in versions.into_iter() {
//         if !config.versions.contains(&version.version) {
//             continue;
//         }
//         dbg!(&version.link);

//         let filename = version.link.split("daily/").nth(1).unwrap();

//         let mut path = PathBuf::from_str(&config.path).unwrap();
//         path.push(filename);

//         if downloaded.contains(&path.with_extension("").with_extension("")) {
//             println!("{} Already at Latest version", version.version);
//             continue;
//         }

//         if path.exists() {
//             println!("{} Already downloaded", version.version);
//             continue;
//         }

//         let mut file = std::fs::File::create(&path).unwrap();

//         let download_result = getter::download(&version.link, &mut file);

//         if download_result.is_err() {
//             println!("Download Error: {}", download_result.err().unwrap());
//         } else {
//             drop(file);

//             extract_and_clean(path, &config);
//         }
//     }
// }

fn parse_downloaded(downloaded: Vec<PathBuf>) -> Vec<BlenderVersion> {
    let pattern =
        Regex::new(r#"blender-(?<version>\d.\d.\d+)-(?<release>\w+)\+(?<branch>.+?)-(?<os>.+)-"#)
            .unwrap();

    let manual_pattern = Regex::new(r#"blender-(?<version>\d.\d.\d+)-(?<os>.+)"#).unwrap();

    let result: Vec<BlenderVersion> = downloaded
        .into_iter()
        .map(|path| {
            let Some(dir_name) = path.components().last() else {
                return None;
            };

            let dir_name = dir_name.as_os_str().to_str().unwrap();

            if let Some(captures) = pattern.captures(dir_name) {
                let version = captures.name("version");
                let release = captures.name("release");
                let branch = captures.name("branch");
                let os = captures.name("os");

                if let (Some(version), Some(release), Some(branch), Some(os)) =
                    (version, release, branch, os)
                {
                    let blender_version = BlenderVersion {
                        version: version.as_str().to_owned(),
                        release: release.as_str().to_owned(),
                        branch: branch.as_str().to_owned(),
                        os: os.as_str().to_owned(),
                        link: "".to_owned(),
                    };

                    return Some(blender_version);
                }
            } else if let Some(captures) = manual_pattern.captures(dir_name) {
                let version = captures.name("version");
                let os = captures.name("os");

                if let (Some(version), Some(os)) = (version, os) {
                    let blender_version = BlenderVersion {
                        version: version.as_str().to_owned(),
                        os: os.as_str().to_owned(),
                        release: "stable".to_owned(),
                        branch: String::new(),
                        link: String::new(),
                    };

                    return Some(blender_version);
                }
            }

            None
        })
        .flatten()
        .collect();

    result
}

async fn main_async() {
    let config = parse_config().unwrap();
    let downloaded = check_downloaded(&config).unwrap();

    let downloaded = parse_downloaded(downloaded);

    let mut app = TuiApp::new(config, downloaded);
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
