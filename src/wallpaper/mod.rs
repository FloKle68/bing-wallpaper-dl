use serde::{Deserialize, Serialize};

pub const PATTERN: &str = r"(?:\.)([0-9A-Za-z]+)(?:_)";
pub const BASE_URL: &str = "https://www.bing.com";
pub const BING_API: &str = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=fr-FR";

#[derive(Serialize, Deserialize)]
pub struct BingResponse {
    pub images: Vec<BingImages>,
    pub tooltips: BingTooltips,
}

#[derive(Serialize, Deserialize)]
pub struct BingImages {
    pub startdate: String,
    pub fullstartdate: String,
    pub enddate: String,
    pub url: String,
    pub urlbase: String,
    pub copyright: String,
    pub copyrightlink: String,
    pub title: String,
    pub quiz: String,
    pub wp: bool,
    pub hsh: String,
    pub drk: u32,
    pub top: u32,
    pub bot: u32,
    pub hs: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BingTooltips {
    pub loading: String,
    pub previous: String,
    pub next: String,
    pub walle: String,
    pub walls: String,
}
