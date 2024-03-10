use std::fs::{remove_file, File};
use std::io::Write;
use std::time::Instant;

use serde_json::{json, Value};
use ureq::Request;

use crate::io::save_results;
use crate::select::{self};
use crate::select::{Album, Media};

use anyhow::Result;

use colored::Colorize;

const COOKIE: &str = "disclaimer=eyJpdiI6ImNTUVhjQmhIR0hXUGlxVStLS0xaSlE9PSIsInZhbHVlIjoiY0pvRlZ6QmMxSDFMbmdKam1wVWVmUT09IiwibWFjIjoiMzFiNzMxOTYzOTllNjMzN2EwYTE0OWE1ZDNiNGViZjVhZTc5ZTIzM2VmMTkwNTU0MDViODYxNGE4OWNiMmMzYiJ9; remember_web_59ba36addc2b2f9401580f014c7f58ea4e30989d=eyJpdiI6IjI4UVRaVmhiZ09KUlNkUm9KZURTOHc9PSIsInZhbHVlIjoiWHhZeE1lTCt2TW5IRHQrdVVqUkIzM3lrSURyaFhNaDJyVmtYdnQrMkR2aDRMdVhMYUI5ajI5RWZNSjdTUm5pWXZEUlc1TDFRNjhBSTNhelp2RzZrVDdnWDhCbFNERzNva1VYZENHNjkybVU9IiwibWFjIjoiZjY1MTQ4N2M0MDg2YWE2ZTk2MjkwN2Y5MGE2ZDFjMTA0NjdmYmZhZjc5MTNmNmMzZDdmOGQ2ZmJkM2RjODQ4OCJ9; XSRF-TOKEN=eyJpdiI6IlwvKzNcL1lUSGlxZ2FVOW8ycWZxMzJrdz09IiwidmFsdWUiOiJvOE8rN2txZlFuVk5uTHU0dnhoeWU5clhRSUUzXC9UNzl1YW1wdXRnSnF4Ym1GcjdyTTNHXC9tOURQMjlYM2VrWlh0d3dqUGlHSDdvd3BYXC9IaTNPWDhHQT09IiwibWFjIjoiM2U3YTU5ZTFkZmI0MGY2ODk1NTI5MDdlMmYzYTllZDMxZTM1NTc1OTY1YmM1MjRiMDVhMjYyN2Y4NTFmZTZmNSJ9; laravel_session=eyJpdiI6IjZ3azVVeDJPanVEa2FOK2NYb0hIbnc9PSIsInZhbHVlIjoiTHZvU1hmNSs0OWdqdmx3RkJGTkRmbWs2QWFPWG1JQTluQ0V6ZStQRWtRQlROUnkyZ2VHS09vQ1NpUnlPeTNKdzhOaW8zUVFWdjdhTE41Z1ZaYzdBR0E9PSIsIm1hYyI6IjU1NDBjZjFhMzQ4NjIyZTdhODk3NmYzM2U4NTI3MmUxM2ZjMmMzYWI5MWJiZTAyYjZiM2FkNGFhMjU2YWIzNDIifQ%3D%3D; _ga_6S5PBWQ8CG=GS1.1.1707261381.1.0.1707261381.0.0.0; _ga=GA1.1.686881403.1707261382";

struct Getter {
    request: Request,
}

impl Getter {
    fn new(url: &str) -> Self {
        let headers = json!(
            {
             "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
             "accept-language": "en-US,en;q=0.9",
             "cache-control": "no-cache",
             "pragma": "no-cache",
             "sec-ch-ua": "\"Not A(Brand\";v=\"99\", \"Google Chrome\";v=\"121\", \"Chromium\";v=\"121\"",
             "sec-ch-ua-mobile": "?0",
             "sec-ch-ua-platform": "\"Linux\"",
             "sec-fetch-dest": "document",
             "sec-fetch-mode": "navigate",
             "sec-fetch-site": "same-origin",
             "sec-fetch-user": "?1",
             "upgrade-insecure-requests": "1",
             "cookie": COOKIE,
             "Referer": "https://www.erome.com/explore",
             "Referrer-Policy": "strict-origin-when-cross-origin"
           }
        );

        let mut request = ureq::get(url);

        for (key, value) in headers.as_object().unwrap() {
            request = request.set(key, value.as_str().unwrap());
        }

        Getter { request }
    }
}

fn get_url(page: u16) -> String {
    return format!("https://www.erome.com/user/saved?page={}", page);
}

pub fn get_saved() {
    let mut results: Vec<Album> = Vec::with_capacity(15 * 16);

    for page in 1..=16 {
        println!("Getting Page {}", page);

        let getter = Getter::new(&get_url(page));

        let r = match getter.request.call() {
            Ok(r) => r,
            Err(err) => {
                println!("Error getting Request");
                dbg!(err.to_string());
                return;
            }
        };

        let body = r.into_string().unwrap();

        let albums = select::select(body).unwrap();
        results.extend(albums);
    }

    save_results(results).unwrap();
}

pub fn get_album(album: &Album) -> Result<String> {
    let getter = Getter::new(&album.link);

    let r = match getter.request.call() {
        Ok(r) => r,
        Err(err) => {
            println!("Error getting Request");
            dbg!(err.to_string());
            return Err(err.into());
        }
    };

    let body = r.into_string()?;

    Ok(body)
}

pub fn download(link: &str, file: &mut File) -> Result<usize> {

    const SIZE_LIMIT: f32 = 260.0;

    let getter = Getter::new(link);

    let r = getter.request.call()?;

    assert!(r.has("Content-Length"));

    let len: usize = r.header("Content-Length").unwrap().parse().unwrap();
    let len_mb = len as f32 / 1000000.0;

    println!("{} {len_mb:.1}mb ({len} bytes)", "Content Size".blue());

    if len_mb > SIZE_LIMIT {
        println!("{} {:.1} : {:.1}mb, skipping...", "Content > limit".red(), SIZE_LIMIT, len_mb);
        return Err(anyhow::anyhow!("TOO_BIG"));
    }

    let mut reader = r.into_reader();

    let mut buffer = [0u8; 1_000_000];

    let mut total_read = 0usize;
    let mut last_read = 0usize;

    let mut start = Instant::now();

    let mut now = Instant::now();
    let mut elapsed;

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {

            let total_elapsed = start.elapsed().as_secs();
            if total_read == len {
                println!(
                    "| {}. Downloaded {len_mb:.1}mb ({total_read} bytes)",

                    "Download Finished".green()
                );
                std::io::stdout().flush();
            } else {
                println!("{}: {total_read}/{len}", "Break before finishing".red());
                std::io::stdout().flush();
            }
            break;
        }

        file.write(&buffer[..n]).unwrap();
        total_read += n;

        elapsed = now.elapsed();

        if elapsed.as_millis() >= 1000 {
            let incremental = total_read - last_read;
            last_read = total_read;
            let percentage = (total_read as f32 / len as f32) * 100.0;
            let kbs = incremental / 1000;
            let total_elapsed = start.elapsed().as_secs();

            print!("\r{percentage:>5.1}% | {kbs:>5} kb/s | {total_elapsed}s ");
            std::io::stdout().flush();

            now = Instant::now();
        }
    }

    if total_read != len {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    Ok(total_read)
}
