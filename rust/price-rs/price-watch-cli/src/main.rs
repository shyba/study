use reqwest;
use mercado_rs::types::SearchResults;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let res = client.get("https://api.mercadolibre.com/sites/MLB/search?q=usb").send().await.unwrap();
    let res = res.json::<SearchResults>().await.unwrap();
    println!("{:?}", res);
    println!("Hello, world!");
}
