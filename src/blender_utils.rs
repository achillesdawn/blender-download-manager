use std::collections::HashMap;

use regex::Regex;
use scraper::{Html, Selector};

use crate::BlenderVersion;

pub struct BlenderMatcher {
    main_pattern: Regex,
    extracted_pattern: Regex,
}

impl BlenderMatcher {
    pub fn new() -> Self {
        let main_pattern = Regex::new(
            r#"blender-(?<version>\d.\d.\d+)-(?<release>\w+)\+(?<branch>.+?)-(?<os>.+)-"#,
        )
        .unwrap();

        let extracted_pattern = Regex::new(r#"blender-(?<version>\d.\d.\d+)-(?<os>.+)"#).unwrap();

        BlenderMatcher {
            main_pattern,
            extracted_pattern,
        }
    }

    pub fn match_str(&self, blender_str: &str) -> Option<BlenderVersion> {
        if let Some(captures) = self.main_pattern.captures(blender_str.into()) {
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
                    link: "".to_owned(),
                };

                return Some(blender_version);
            }
        } else if let Some(captures) = self.extracted_pattern.captures(blender_str) {
            let version = captures.name("version");
            let os = captures.name("os");

            if let (Some(version), Some(os)) = (version, os) {
                let blender_version = BlenderVersion {
                    version: version.as_str().to_owned(),
                    os: os.as_str().to_owned(),
                    release: "stable".to_owned(),
                    branch: String::new(),
                    link: String::new(),
                };

                return Some(blender_version);
            }
        }

        None
    }
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

pub fn select(body: String) -> Result<Vec<BlenderVersion>, String> {
    let document = Html::parse_document(&body);

    let selector =
        Selector::parse("[data-platform='linux'] li:not([style='display:none;']) a:first-child")
            .unwrap();

    let mut links = Vec::with_capacity(600);

    let matcher = BlenderMatcher::new();

    for el in document.select(&selector) {
        let href = el.attr("href").unwrap();

        if let Some(mut blender_version) = matcher.match_str(href) {
            blender_version.link = href.to_owned();
            links.push(blender_version);
        }
    }

    let links = filter_latest(links);

    Ok(links)
}
