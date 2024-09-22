use std::time::Instant;

use alloy_core::primitives::{address, b256};
use fancy_skinny_create2::salt_bruteforce;

fn main() {
    let sender = address!("0000000000000000000000000000000000000000");
    let init_code_hash = b256!("0000000000000000000000000000000000000000000000000000000000000000");
    let pattern = address!("0000000000000000000000000000000000000000");
    let mask = address!("ffff000000000000000000000000000000000000");

    let bruteforce_time = Instant::now();
    if let Ok((address, salt)) = salt_bruteforce(16, sender, init_code_hash, pattern, mask) {
        println!("Address: {:?}, Salt: {:?} (in {:?})", address, salt, bruteforce_time.elapsed());
    } else {
        panic!("Communication failure during bruteforce.");
    }
}