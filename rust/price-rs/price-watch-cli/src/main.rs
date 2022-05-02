use reqwest;
use reqwest::Client;
use mercado_rs::types::{SearchResults, RequestBuilder};

struct SearchClient {
    reqwest_client: Client,
    builder: RequestBuilder
}

impl SearchClient {
    fn new() -> SearchClient {
        SearchClient {reqwest_client: reqwest::Client::new(), builder: RequestBuilder::new()}
    }

    async fn search(&self, term: &str) -> SearchResults {
        let url = self.builder.build_query_url(term);
        let res = self.reqwest_client.get(url).send().await.unwrap();
        res.json::<SearchResults>().await.unwrap()
    }
}

#[tokio::main]
async fn main() {
    let client = SearchClient::new();
    println!("{:?}", client.search("usb").await);
    println!("Hello, world!");
}
