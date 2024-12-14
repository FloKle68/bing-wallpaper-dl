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

#[warn(dead_code)]
enum Vendeur {
    EN_US,
    ZH_CN,
    JA_JP,
    EN_AU,
    EN_GB,
    DE_DE,
    EN_NZ,
    EN_CA,
    EN_IN,
    FR_FR,
    FR_CA,
}
