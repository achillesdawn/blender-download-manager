#![allow(unused)]
use std::{fs, path::PathBuf, str::FromStr};

use select::Album;


mod select;
mod io;
mod get;


fn main() {

    let data = io::load_results().unwrap();
    
    for (idx, album) in data.iter().enumerate() {
        let mut path = PathBuf::from_str("/mnt/novaera/rust/downloader/test").unwrap();

        if let Some(title) = &album.title {
            path.push(title);    
        } else {
            path.push(idx.to_string());
        }
        
        if !path.exists() {
            fs::create_dir(&path);
        }

        let file = fs::File::create(path);
        let body = get::get_album(album).unwrap();
        select::get_album(body);
        break;
    }
}