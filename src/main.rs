use anyhow::Error;
use log::LevelFilter;
use regex::Regex;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

fn main() -> anyhow::Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Debug).init()?;

    let base_url = "https://www.bing.com";
    let url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=fr-FR";
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let bing_response: BingResponse = serde_json::from_str(&response)?;
    let var_url = &bing_response.images[0].url.as_str();

    let title =
        extract_filename(&bing_response.images[0].urlbase.as_str()).expect("No title found.");
    let startdate = &bing_response.images[0].startdate.as_str();

    let file_name = format!("{startdate}-{title}.png");

    let mut file = std::fs::File::create("img/".to_owned()+file_name.as_str() + ".png")?;

    reqwest::blocking::get(base_url.to_owned() + var_url)?.copy_to(&mut file)?;

    Ok(())
}

fn extract_filename(input: &str) -> Option<&str> {
    log::debug!("{input}");
    let re = Regex::new(r"(?:\.)([0-9A-Za-z]+)(?:_)").expect("Unable to initiate regex");
    let caps = re
        .captures(input)
        .unwrap_or_else(|| panic!("No value found for title in : {input}"));

    //Group 0 is the whole capture, with . and _ included.
    //Group 1 is the correct capture. There should not be any more capture than that.
    if caps.len() > 2 {
        log::warn!(
            "Error parsing the url, too many potential filename : {:?}",
            caps
        )
    }

    let filename = caps.get(1).unwrap().as_str();
    log::debug!("{}", filename);
    Some(filename)
}

#[derive(Serialize, Deserialize)]
struct BingResponse {
    images: Vec<BingImages>,
    tooltips: BingTooltips,
}

#[derive(Serialize, Deserialize)]
struct BingImages {
    startdate: String,
    fullstartdate: String,
    enddate: String,
    url: String,
    urlbase: String,
    copyright: String,
    copyrightlink: String,
    title: String,
    quiz: String,
    wp: bool,
    hsh: String,
    drk: u32,
    top: u32,
    bot: u32,
    hs: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct BingTooltips {
    loading: String,
    previous: String,
    next: String,
    walle: String,
    walls: String,
}
