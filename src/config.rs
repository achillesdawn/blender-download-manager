use std::{io::Read, path::PathBuf, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub versions: Vec<String>,
    pub path: String,
    pub archive: Option<bool>,
    #[serde(default)]
    pub link: String,
}

pub fn parse_config() -> anyhow::Result<Config> {
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
