use std::path::PathBuf;

mod getter;
mod select;

fn check_downloaded() -> anyhow::Result<Vec<PathBuf>> {
    let path = PathBuf::from("/home/miguel/blenders");

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
fn main() {
    let downloaded = check_downloaded().unwrap();

    let versions = ["4.1.1"];

    let links = getter::get_links().unwrap();

    for (key, link) in links.into_iter() {
        if versions.contains(&key.as_str()) {
            dbg!(&link);

            let filename = link
                .split("https://builder.blender.org/download/daily/")
                .nth(1)
                .unwrap();

            let mut path = PathBuf::from("/home/miguel/blenders/");
            path.push(filename);

            if downloaded.contains(&path.with_extension("").with_extension("")) {
                println!("{} Already at Latest version", key);
                continue;
            }

            let mut file = std::fs::File::create(path).unwrap();

            let download_result = getter::download(&link, &mut file);

            if download_result.is_err() {
                println!("Download Error: {}", download_result.err().unwrap());
            }
        }
    }
}
