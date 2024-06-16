use std::fs::File;
use std::io::Write;

use serde_json::json;
use ureq::Request;

use anyhow::Result;

use colored::Colorize;

use crate::{
    select::{self, BlenderVersion},
    Config,
};

use crate::tracker::ProgressTracker;

struct Getter {
    request: Request,
}

impl Getter {
    fn new(url: &str) -> Self {
        let headers = json!( {
              "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
              "accept-language": "en-US,en;q=0.9,es-419;q=0.8,es;q=0.7,pt;q=0.6,fr;q=0.5",
              "cache-control": "no-cache",
              "pragma": "no-cache",
              "sec-ch-ua": "\"Google Chrome\";v=\"123\", \"Not:A-Brand\";v=\"8\", \"Chromium\";v=\"123\"",
              "sec-ch-ua-mobile": "?0",
              "sec-ch-ua-platform": "\"Linux\"",
              "sec-fetch-dest": "document",
              "sec-fetch-mode": "navigate",
              "sec-fetch-site": "same-site",
              "sec-fetch-user": "?1",
              "upgrade-insecure-requests": "1",
              "cookie": "_ga_MCBBT8QGSN=GS1.1.1706631626.3.0.1706631628.0.0.0; _ga=GA1.2.753018188.1701969904; _ga_L6Q2GW7H9J=GS1.2.1708030780.35.0.1708030780.0.0.0",
              "Referer": "https://www.blender.org/download/",
              "Referrer-Policy": "no-referrer-when-downgrade",
              "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
            }
        );

        let mut request = ureq::get(url);

        for (key, value) in headers.as_object().unwrap() {
            request = request.set(key, value.as_str().unwrap());
        }

        Getter { request }
    }
}

pub fn get_links(config: &Config) -> anyhow::Result<Vec<BlenderVersion>> {
    let getter = Getter::new(&config.link);

    let r = match getter.request.call() {
        Ok(r) => r,
        Err(err) => {
            println!("Error getting Request");
            dbg!(err.to_string());
            return Err(err.into());
        }
    };

    let body = r.into_string()?;
    select::select(body)
}

pub fn download(link: &str, file: &mut File) -> Result<usize> {
    let getter = Getter::new(link);

    let r = getter.request.call()?;

    assert!(r.has("Content-Length"));

    let len: usize = r.header("Content-Length").unwrap().parse().unwrap();
    let len_mb = len as f32 / 1000000.0;

    println!("{} {len_mb:.1}mb ({len} bytes)", "Content Size".blue());

    let mut reader = r.into_reader();

    let mut buffer = [0u8; 1_000_000];

    let mut tracker = ProgressTracker::new(len);

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {

            tracker.flush();

            if tracker.total_read == len {
                println!(
                    "| {}. Downloaded {len_mb:.1}mb ({} bytes)",
                    "Download Finished".green(),
                    tracker.total_read
                );
                std::io::stdout().flush().unwrap();
            } else {
                println!(
                    "{}: {}/{len}",
                    "Break before finishing".red(),
                    tracker.total_read
                );
                std::io::stdout().flush().unwrap();
            }
            break;
        }

        file.write(&buffer[..n]).unwrap();

        tracker.update(n);
    }

    if tracker.total_read != len {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    Ok(tracker.total_read)
}
