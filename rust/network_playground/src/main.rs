use network_playground::tracker_types::TrackerRequestBuilder;
use core::sync::atomic::AtomicU32;

fn main() {

    let builder = TrackerRequestBuilder {
        random_seed: AtomicU32::new(42),
        connection_id: None
    };
    println!("{:?}", builder.connect());
}