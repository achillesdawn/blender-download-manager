use std::path::PathBuf;

use select::BlenderVersion;

use colored::Colorize;
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

async fn main_async() {
    let config = parse_config().unwrap();
    let downloaded = check_downloaded(&config).unwrap();

    let downloaded: Vec<String> = downloaded
        .into_iter()
        .map(|path| path.as_os_str().to_str().unwrap().to_owned())
        .collect();

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
