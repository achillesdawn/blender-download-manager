#![allow(unused)]
use std::{fs, path::PathBuf, str::FromStr};

use select::{Album, Media};

mod get;
mod io;
mod select;

fn main() {
    let data = io::load_results().unwrap();

    for (idx, album) in data.iter().enumerate() {

        println!("Getting: {:?}", album.link);

        let mut path = PathBuf::from_str("/home/miguel/rust/downloader/test").unwrap();

        if let Some(title) = &album.title {
            path.push(title);
        } else {
            path.push(idx.to_string());
        }

        if !path.exists() {
            fs::create_dir(&path);
        }

        let body = get::get_album(album).unwrap();
        let media = select::get_album_media(body).unwrap();


        for (idx, item) in media.into_iter().enumerate() {
            let mut media_path = path.clone();
            media_path.push(idx.to_string());

            match &item {
                Media::Photo(link) => {
                    media_path = media_path.with_extension("png");
                    let mut file = fs::File::create(&media_path).unwrap();
                    get::download(link, &mut file);
                }
                Media::Video(link) => {
                    media_path = media_path.with_extension("mp4");
                    let mut file = fs::File::create(&media_path).unwrap();
                    get::download(link, &mut file);
                }
            }
        }
    }
}
