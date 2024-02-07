#![allow(unused)]
use std::{
    fs::{self, remove_file},
    path::PathBuf,
    str::FromStr,
};

use select::{Album, Media};

mod get;
mod io;
mod select;

use colored::Colorize;
fn main() {
    let data = io::load_results().unwrap();

    for (idx, album) in data.iter().enumerate() {
        println!("Getting: {:?}", album.link);

        let mut path = PathBuf::from_str("/home/miguel/rust/downloader/test").unwrap();

        if let Some(title) = &album.title {
            let title = title.replace("/", "");
            println!("{}",title.yellow());
            path.push(title);
        } else {
            path.push(idx.to_string());
        }

        if !path.exists() {
            fs::create_dir(&path);
        } else {
            println!("{} skipping..", path.to_str().unwrap().blue());
            continue;
        }

        let body = get::get_album(album).unwrap();
        let media = select::get_album_media(body).unwrap();

        let mut video_count = 0u16;

        for (idx, item) in media.into_iter().enumerate() {
            let mut media_path = path.clone();
            media_path.push(idx.to_string());

            match &item {
                Media::Photo(link) => {
                    media_path = media_path.with_extension("png");
                    let mut file = fs::File::create(&media_path).unwrap();
                    if let Err(err) = get::download(link, &mut file) {
                        if err.to_string() == "TOO_BIG" {
                            remove_file(media_path);
                        }
                    }
                }
                Media::Video(link) => {
                    if video_count > 3 {
                        println!("{} {video_count}", "Video count greater than 3: ".red());
                        continue;
                    }
                    media_path = media_path.with_extension("mp4");
                    let mut file = fs::File::create(&media_path).unwrap();
                    if let Err(err) = get::download(link, &mut file) {
                        if err.to_string() == "TOO_BIG" {
                            remove_file(media_path);
                        }
                    }
                    video_count += 1;
                }
            }
        }
    }
}
