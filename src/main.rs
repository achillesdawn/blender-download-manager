use std::path::PathBuf;

use downloader::{
    blender_utils::BlenderMatcher,
    config::{parse_config, Config},
    BlenderVersion,
};

use downloader::tui::TuiApp;

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
    let matcher = BlenderMatcher::new();

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

async fn main_async() {
    let config = parse_config().unwrap();
    let downloaded = check_downloaded(&config).unwrap();

    let downloaded = parse_downloaded(downloaded);

    let mut app = TuiApp::new(config, downloaded);
    let mut terminal = downloader::tui::init().unwrap();
    app.run(&mut terminal).await.unwrap();
    downloader::tui::restore().unwrap();
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(main_async());
}
