use crate::types::RequestBuilder;

pub mod types;


impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder {base_url: "https://api.mercadolibre.com/sites/MLB/search".to_string()}
    }
    pub fn build_query_url(&self, term: &str) -> String {
        format!("{}?q={}", self.base_url, term)
    }
}

#[cfg(test)]
mod tests {
    use crate::RequestBuilder;

    #[test]
    fn basic_query_url() {
        let result = RequestBuilder::new().build_query_url("product");
        assert_eq!(result, "https://api.mercadolibre.com/sites/MLB/search?q=product");
    }
}
