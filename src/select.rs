use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub link: String,
    pub title: Option<String>,
    pub photos: Option<u32>,
    pub videos: Option<u32>,
}

#[derive(Debug)]
pub enum Media {
    Photo(String),
    Video(String),
}

pub fn select(body: String) -> Result<Vec<Album>, Box<dyn Error>> {
    let document = Html::parse_document(&body);

    let selector = Selector::parse(".album-link").unwrap();
    let im_selector = Selector::parse("img").unwrap();
    let photos_selector = Selector::parse(".album-images").unwrap();
    let videos_selector = Selector::parse(".album-videos").unwrap();

    let mut result: Vec<Album> = Vec::with_capacity(20);

    for el in document.select(&selector) {
        let Some(link) = el.attr("href") else {
            continue;
        };

        let mut album = Album {
            link: link.to_owned(),
            title: None,
            photos: None,
            videos: None,
        };

        if let Some(title) = el
            .select(&im_selector)
            .next()
            .and_then(|child| child.attr("alt"))
        {
            album.title = Some(title.to_owned());
        }

        if let Some(photos) = el
            .select(&photos_selector)
            .next()
            .and_then(|child| Some(child.text().collect::<Vec<&str>>()))
        {
            if let Ok(n) = photos.first().unwrap().parse() {
                album.photos = Some(n);
            }
        }

        if let Some(videos) = el
            .select(&videos_selector)
            .next()
            .and_then(|child| Some(child.text().collect::<Vec<&str>>()))
        {
            if let Ok(n) = videos.first().unwrap().parse() {
                album.videos = Some(n);
            }
        }

        result.push(album);
    }

    Ok(result)
}

pub fn get_album(body: String) {
    let document = Html::parse_document(&body);

    let selector = Selector::parse(".media-group").unwrap();
    let video_selector = Selector::parse("source").unwrap();
    let photo_selector = Selector::parse("img").unwrap();

    for el in document.select(&selector) {
        if let Some(video) = el.select(&video_selector).next() {
            let video = Media::Video(video.attr("src").unwrap().to_owned());

            dbg!(video);
        } else {
            if let Some(photo) = el.select(&photo_selector).next() {
                let photo = Media::Photo(photo.attr("data-src").unwrap().to_owned());

                dbg!(photo);
            }
        }
    }
}
