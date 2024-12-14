use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BingResponse {
    pub images: Vec<BingImages>,
}

#[derive(Serialize, Deserialize)]
pub struct BingImages {
    pub startdate: String,
    pub url: String,
    pub urlbase: String,
    pub title: String,
}
