use std::{error::Error, fs, io::{BufReader, BufWriter}, path::PathBuf, str::FromStr};

use crate::select::Album;

pub fn save_results(albums: Vec<Album>) -> Result<(), Box<dyn Error>> {

    // let path = PathBuf::from_str("results.json").unwrap();

    // if path.exists(){
    //     let previous = fs::File::open("results.json").unwrap();

    //     let new = fs::File::create(path)
    //     previous.

    // }


    let file = fs::File::create("results.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &albums)?;
    Ok(())
}

pub fn load_results() -> Result<Vec<Album>, Box<dyn Error>> {

    let file = fs::File::open("results.json").unwrap();
    let mut reader = BufReader::new(file);
    let data: Vec<Album> = serde_json::from_reader(reader).unwrap();

    Ok(data)
}
