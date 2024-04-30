use futures::{
    stream::{FuturesOrdered, FuturesUnordered},
    StreamExt,
};
use tokio::time::{sleep, Duration};

async fn sleep_and_do<F, T>(sleep_ms: u64, do_what: F) -> T
where
    F: FnOnce() -> T,
{
    sleep(Duration::from_millis(sleep_ms)).await;
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
    println!("sorted: {:?}", sort(original.clone()).await);
    println!("sorted(broken): {:?}", sort_broken(original.clone()).await);
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
