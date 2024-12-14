#[derive(Debug, PartialEq)]
pub struct BingRequest {
    pub base_url: String,
    pub format: String,
    pub idx: usize,
    pub number_of_days: usize,
    pub market: String,
}

impl BingRequest {
    pub fn builder() -> BingRequestBuilder {
        BingRequestBuilder::default()
    }

    pub fn get_string(&self) -> String {
        format!(
            "{}format={}&idx={}&n={}&mkt={}",
            self.base_url.to_owned(),
            self.format.to_owned(),
            self.idx,
            self.number_of_days,
            self.market.to_owned()
        )
    }
}

pub struct BingRequestBuilder {
    pub base_url: String,
    pub format: String,
    pub idx: usize,
    pub number_of_days: usize,
    pub market: String,
}

impl BingRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(mut self, format: String) -> Self {
        self.format = format;
        self
    }

    pub fn idx(mut self, idx: usize) -> Self {
        self.idx = idx;
        self
    }

    pub fn number_of_days(mut self, number_of_days: usize) -> Self {
        self.number_of_days = number_of_days;
        self
    }

    pub fn market(mut self, market: String) -> Self {
        self.market = market;
        self
    }

    pub fn build(self) -> BingRequest {
        BingRequest {
            base_url: self.base_url,
            format: self.format,
            idx: self.idx,
            number_of_days: self.number_of_days,
            market: self.market,
        }
    }
}

impl Default for BingRequestBuilder {
    fn default() -> Self {
        Self {
            base_url: String::from("https://www.bing.com/HPImageArchive.aspx?"),
            format: String::from("js"),
            idx: 0,
            number_of_days: 1,
            market: String::from("fr-FR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_get_string() {
        let bing_request = BingRequestBuilder::new().build();
        println!("---------- {} --------", bing_request.base_url.as_str());

        let expected_url =
            String::from("https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=fr-FR");

        assert_eq!(bing_request.get_string(), expected_url);
    }

    #[test]
    fn builder_test_base() {
        let bing_request = BingRequest {
            base_url: String::from("https://www.bing.com/HPImageArchive.aspx?"),
            format: String::from("js"),
            idx: 0,
            number_of_days: 1,
            market: String::from("fr-FR"),
        };

        let bing_request_from_builder = BingRequestBuilder::new().build();

        assert_eq!(bing_request, bing_request_from_builder);
    }

    #[test]
    fn builder_test_modification() {
        let bing_request = BingRequest {
            base_url: String::from("https://www.bing.com/HPImageArchive.aspx?"),
            format: String::from("xml"),
            idx: 1,
            number_of_days: 1,
            market: String::from("en-US"),
        };

        let bing_request_from_builder = BingRequestBuilder::new()
            .format(String::from("xml"))
            .idx(1)
            .number_of_days(1)
            .market(String::from("en-US"))
            .build();

        assert_eq!(bing_request, bing_request_from_builder);
    }
}
