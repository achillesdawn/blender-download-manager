use std::collections::HashMap;
use regex::Regex;
use scraper::{Html, Selector};

pub fn select(body: String) -> anyhow::Result<HashMap<String, String>> {
    let document = Html::parse_document(&body);

    let selector =
        Selector::parse(".platform-linux li:not([style='display:none;']) a:first-child").unwrap();

    let mut links: HashMap<String, String> = HashMap::with_capacity(10);

    let pattern = Regex::new(r"/blender-(\d.\d.\d+)-")?;

    for el in document.select(&selector) {
        let href = el.attr("href").unwrap();

        if let Some(captures) = pattern.captures(href) {
            captures.get(1).inspect(|version| {
                links.insert(version.as_str().to_owned(), href.to_owned());
            });
        }
    }

    Ok(links)
}
