use std::time::Duration;

use futures::{stream::FuturesOrdered, StreamExt};
use tokio::time::sleep;

async fn sleep_and_do<F, T>(sleep_seconds: u64, do_what: F) -> T where F: FnOnce() -> T {
    sleep(Duration::from_millis(sleep_seconds)).await;
    do_what()
}

async fn sort(vals: Vec<u64>) -> Vec<u64>{
    let futs: FuturesOrdered<_> = vals.iter().map(|val| {sleep_and_do(*val, move ||{val})}).collect();
    futs.collect().await
}


#[tokio::main]
async fn main() {
    println!("{:?}", sort(vec![10, 0, 2, 6]).await)
}