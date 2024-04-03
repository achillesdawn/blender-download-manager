use std::path::PathBuf;

mod getter;
mod select;

fn main() {
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

            let mut file = std::fs::File::create(path).unwrap();
            
            let download_result = getter::download(&link, &mut file);

            if download_result.is_err() {
                println!("Download Error: {}", download_result.err().unwrap());
            }
        }
    }
}
