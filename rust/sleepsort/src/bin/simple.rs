use futures::{
    stream::{FuturesOrdered, FuturesUnordered},
    StreamExt,
};
use tokio::time::{sleep, Duration, Instant};

async fn sleep_and_do<F, T>(sleep_seconds: u64, do_what: F) -> T
where
    F: FnOnce() -> T,
{
    sleep(Duration::from_millis(sleep_seconds)).await;
    do_what()
}

async fn sort(vals: Vec<u64>) -> Vec<u64> {
    let futs: FuturesUnordered<_> = vals
        .iter()
        .map(|val| sleep_and_do(*val, move || val))
        .collect();
    futs.collect().await
}

async fn sort_broken(vals: Vec<u64>) -> Vec<u64> {
    let futs: FuturesOrdered<_> = vals
        .iter()
        .map(|val| sleep_and_do(*val, move || val))
        .collect();
    futs.collect().await
}

#[tokio::main]
async fn main() {
    let original = vec![10, 0, 2, 6];
    println!("original: {original:?}");
    println!("sort: {:?}", sort(original.clone()).await);
    println!("broken sort: {:?}", sort_broken(original.clone()).await);
    let start = Instant::now();
    sort((0..1000).rev().collect()).await;
    println!("{:?}", Instant::now() - start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn paused_time() {
        tokio::time::pause();
        let start = Instant::now();
        sort((0..100000).rev().collect()).await;
        println!("took {:?}", Instant::now() - start);
    }
}
