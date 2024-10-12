use std::collections::HashMap;

use regex::Regex;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct BlenderVersion {
    pub version: String,
    pub release: String,
    pub branch: String,
    pub os: String,
    pub link: String,
}

fn filter_latest(versions: Vec<BlenderVersion>) -> Vec<BlenderVersion> {
    let mut result = HashMap::new();

    for version in versions.into_iter().rev() {
        let key = (version.version.clone(), version.release.clone());

        if result.contains_key(&(version.version.clone(), "stable".to_string())) {
            continue;
        } else if result.contains_key(&key) {
            continue;
        } else {
            result.insert(key, version);
        }
    }

    let mut result: Vec<BlenderVersion> = result.into_values().into_iter().collect();
    result.sort_by(|a, b| b.version.partial_cmp(&a.version).unwrap());
    result
}

pub fn select(body: String) -> anyhow::Result<Vec<BlenderVersion>> {
    let document = Html::parse_document(&body);

    let selector =
        Selector::parse("[data-platform='linux'] li:not([style='display:none;']) a:first-child")
            .unwrap();

    let mut links = Vec::with_capacity(600);

    let pattern =
        Regex::new(r#"blender-(?<version>\d.\d.\d+)-(?<release>\w+)\+(?<branch>.+?)-(?<os>.+)-"#)?;

    for el in document.select(&selector) {
        let href = el.attr("href").unwrap();

        if let Some(captures) = pattern.captures(href) {
            let version = captures.name("version");
            let release = captures.name("release");
            let branch = captures.name("branch");
            let os = captures.name("os");

            if let (Some(version), Some(release), Some(branch), Some(os)) =
                (version, release, branch, os)
            {
                let blender_version = BlenderVersion {
                    version: version.as_str().to_owned(),
                    release: release.as_str().to_owned(),
                    branch: branch.as_str().to_owned(),
                    os: os.as_str().to_owned(),
                    link: href.to_owned(),
                };

                links.push(blender_version);
            }
        }
    }

    let links = filter_latest(links);

    Ok(links)
}
