#![no_std]

extern crate alloc;

use alloc::vec::Vec;
//http://xbtt.sourceforge.net/udp_tracker_protocol.html
use core::sync::atomic::{AtomicU32, Ordering};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Copy, Clone)]
enum TrackerActions {
    Connect = 0,
    Announce = 1,
    Scrape = 2
}

const PROTOCOL_MAGIC: u64 = 0x41727101980;

#[derive(Debug)]
pub struct ConnectRequest {
    pub connection_id: u64,
    pub action: u32,
    pub transaction_id: u32,
}

pub struct TrackerRequestBuilder {
    pub random_seed: AtomicU32,
    pub connection_id: Option<u32>,
}

impl TrackerRequestBuilder {
    pub fn connect(&self) -> ConnectRequest {
        ConnectRequest {
            connection_id: PROTOCOL_MAGIC,
            action: TrackerActions::Connect as u32,
            transaction_id: self.random_seed.fetch_add(1, Ordering::Relaxed)
        }
    }

    pub fn write_connect(&self, buf: &mut Vec<u8>) {
        buf.write_u32::<BigEndian>(PROTOCOL_MAGIC as u32).unwrap();
        buf.write_u32::<BigEndian>(TrackerActions::Connect as u32).unwrap();
        buf.write_u32::<BigEndian>(self.random_seed.fetch_add(1, Ordering::Relaxed)).unwrap();
    }

}

#[cfg(test)]
mod tests {
    use core::sync::atomic::AtomicU32;
    extern crate alloc;
    use alloc::collections::BTreeSet;
    use super::{PROTOCOL_MAGIC, TrackerActions};
    use super::TrackerRequestBuilder;

    #[test]
    fn connect_sequence() {
        let initial_value = 10;
        let builder = TrackerRequestBuilder {
            random_seed: AtomicU32::new(initial_value),
            connection_id: None
        };
        let request = builder.connect();
        assert_eq!(PROTOCOL_MAGIC, request.connection_id);
        assert_eq!(TrackerActions::Connect as u32, request.action);

        let id_set: BTreeSet<u32> = BTreeSet::from_iter(
            (0..10).map(|_| builder.connect().transaction_id));
        assert_eq!(id_set.len(), 10);  // we only care it is unique..
    }

}
