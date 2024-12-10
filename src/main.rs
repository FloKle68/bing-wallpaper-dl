mod wallpaper;

use anyhow::Error;
use log::LevelFilter;
use regex::Regex;
use simple_logger::SimpleLogger;
use wallpaper::{BingResponse, BASE_URL, BING_API, PATTERN};

fn main() -> anyhow::Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Debug).init()?;

    let response = reqwest::blocking::get(BING_API).unwrap().text().unwrap();
    let bing_response: BingResponse = serde_json::from_str(&response)?;
    let var_url = &bing_response.images[0].url.as_str();

    log::debug!("{}", &bing_response.images[0].urlbase);

    let title = extract_filename(&bing_response.images[0].urlbase.as_str(), PATTERN)
        .get(0)
        .cloned()
        .expect("blabla");
    let startdate = &bing_response.images[0].startdate.as_str();

    let file_name = format!("{startdate}-{title}.png");

    let mut file = std::fs::File::create("img/".to_owned() + file_name.as_str() + ".png")?;

    reqwest::blocking::get(BASE_URL.to_owned() + var_url)?.copy_to(&mut file)?;

    Ok(())
}

fn extract_filename(input: &str, pattern: &str) -> Vec<String> {
    log::debug!("{input}");
    let re = Regex::new(pattern).expect("Unable to initiate regex");
    let mut matches = Vec::new();
    for (_, [cap]) in re.captures_iter(input).map(|c| c.extract()) {
        matches.push(cap.to_owned());
    }

    log::debug!("Number of captured groups : {}", matches.len());
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_filename() {
        let test_string = "/th?id=OHR.CornwallSnow_FR-FR1834119825";
        let extracted_caps = extract_filename(&test_string, PATTERN);

        assert_eq!(extracted_caps.get(0).unwrap(), "CornwallSnow")
    }
}
