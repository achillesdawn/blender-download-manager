#![allow(unused)]
use std::{
    fs::{self, read_dir, remove_file},
    path::PathBuf,
    str::FromStr,
};

use select::{Album, Media};

mod get;
mod io;
mod select;

use colored::Colorize;
use ureq::Transport;
fn main() {
    get::get_saved();

    let data = io::load_results().unwrap();

    for (idx, album) in data.iter().enumerate() {
        

        let mut path = PathBuf::from_str("/home/miguel/rust/downloader/test").unwrap();

        if let Some(title) = &album.title {
            //sanitize
            let mut title = title.replace("/", "").replace(".", "");

            let album_id = album.link.rsplit("/").next().unwrap().to_string();

            let title = [title, album_id].join("_");

            println!("{}", title.yellow());
            path.push(title);
        } else {
            path.push(idx.to_string());
        }

        if !path.exists() {
            fs::create_dir(&path);
        }

        let mut photo_count = 0u32;
        let mut video_count = 0u32;

        for entry in read_dir(&path).unwrap() {
            let entry = entry.unwrap();

            let path = entry.path();
            let Some(extension) = path.extension() else {
                continue;
            };

            let extension = extension.to_string_lossy();

            if extension == "png" {
                photo_count += 1;
            } else if extension == "mp4" {
                video_count += 1;
            }
        }

        let total = album.photos.unwrap_or(0) + album.videos.unwrap_or(0);

        if total == photo_count + video_count {
            println!(
                "Local: {}/{} photos {}/{} videos",
                photo_count, album.photos.unwrap_or(0), 
                video_count, album.videos.unwrap_or(0)
            );
            continue;
        }


        println!("Getting: {:?}", album.link);

        let body = match get::get_album(album) {
            Ok(body) => body,
            Err(err) => {
                if err.is::<Transport>() {
                    println!("{}", "Connection Reset".red());
                    continue;
                } else {
                    println!("Unknown Error");
                    continue;
                }

                continue;
            },
        };

        let media = select::get_album_media(body).unwrap();

        let mut video_count = 0u16;

        let skip_videos = false;
        let skip_images = false;

        for (idx, item) in media.into_iter().enumerate() {
            let mut media_path = path.clone();

            match &item {
                Media::Photo(link) => {
                    if skip_images {
                        continue;
                    }

                    let media_id = link.rsplit("/").next().unwrap();
                    media_path.push(idx.to_string() + "_" + media_id);
                    media_path = media_path.with_extension("png");

                    if media_path.exists() {
                        continue;
                    }

                    let mut file = fs::File::create(&media_path).unwrap();
                    if let Err(err) = get::download(link, &mut file) {
                        if err.to_string() == "TOO_BIG" {
                            remove_file(media_path);
                        }
                    }
                }
                Media::Video(link) => {
                    if skip_videos {
                        continue;
                    }

                    // if video_count > 3 {
                    //     println!("{} {video_count}", "Video count greater than 3: ".red());
                    //     // continue;
                    // }

                    let media_id = link.rsplit("/").next().unwrap();
                    media_path.push(idx.to_string() + "_" + media_id);
                    media_path = media_path.with_extension("mp4");

                    if media_path.exists() {
                        continue;
                    }

                    let mut file = fs::File::create(&media_path).unwrap();
                    if let Err(err) = get::download(link, &mut file) {
                        if err.to_string() == "TOO_BIG" {
                            remove_file(media_path);
                        } else {
                            dbg!(err);
                        }
                    } else {
                        video_count += 1;
                    }
                }
            }
        }
    }
}
