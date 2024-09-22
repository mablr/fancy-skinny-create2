//! # Fancy-Skinny-Create2
//!
//! This crate provides functionality to brute-force a salt for the Ethereum CREATE2 opcode, 
//! allowing you to find a contract address that matches a given pattern and mask.
//!
//! It uses multi-threading to divide the search space and stop as soon as the correct salt is found.


use std::{fmt::Error, sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc}, thread};

use alloy_core::primitives::{Address, FixedBytes, U256};


/// Attempts to find a matching contract address using the CREATE2 opcode by brute-forcing
/// different salt values within a specified range.
/// 
/// # Arguments
///
/// * `sender` - The deployer's Ethereum address.
/// * `init_code_hash` - The keccak256 hash of the contract's initialization code.
/// * `salt_range` - A tuple representing the range of salts to brute-force.
/// * `pattern` - The desired pattern for the contract address.
/// * `mask` - A mask used to define which bits in the address should match the pattern.
/// * `feedback` - An mpsc channel sender to communicate results back to the main thread.
/// * `stop_flag` - An atomic boolean used to signal other threads to stop when a result is found.
///
/// This function runs in a loop, checking each salt in the provided range. If a matching address
/// is found (i.e., `address & mask == pattern`), the result is sent back via the `feedback` channel.
pub fn find_matching_address(
    sender: Address,
    init_code_hash: FixedBytes<32>,
    salt_range: (U256, U256),
    pattern: Address,
    mask: Address,
    feedback: mpsc::Sender<(Address, FixedBytes<32>)>,
    stop_flag: Arc<AtomicBool>,
) {
    let mut salt = salt_range.0;
    while !stop_flag.load(Ordering::Relaxed) && salt < salt_range.1 {
        let address = sender.create2(FixedBytes::from(salt), init_code_hash);
        if address.bit_and(mask).bit_xor(pattern).is_zero() {
            let _ = feedback.send((address, FixedBytes::from(salt)));
            stop_flag.store(true, Ordering::Relaxed);
            break;
        } else {
            // increment_fixed_bytes(&mut salt);
            salt += U256::from(1);
        }
    }
}

/// Brute-forces the salt for the Ethereum CREATE2 opcode to find a contract address that matches
/// a given pattern and mask, using multiple threads.
///
/// # Arguments
///
/// * `nb_threads` - The number of threads to use for the brute-force operation.
/// * `sender` - The Ethereum address of the contract deployer.
/// * `init_code_hash` - The keccak256 hash of the contract's initialization code.
/// * `pattern` - The desired pattern for the resulting contract address.
/// * `mask` - A bitmask used to match the pattern.
///
/// # Returns
///
/// On success, returns the matching contract address and the salt used to generate it.
/// On failure, returns an error.
///
/// # Example
///
/// ```no_run
/// use alloy_core::primitives::{address, b256};
/// use fancy_skinny_create2::salt_bruteforce;
///
/// let sender = address!("0000000000000000000000000000000000000000");
/// let init_code_hash = b256!("0000000000000000000000000000000000000000000000000000000000000000");
/// let pattern = address!("0000000000000000000000000000000000000000");
/// let mask = address!("ffff000000000000000000000000000000000000");
///
/// if let Ok((address, salt)) = salt_bruteforce(16, sender, init_code_hash, pattern, mask) {
///     println!("Found matching address: {:?} with salt: {:?}", address, salt);
/// }
/// ```
pub fn salt_bruteforce(
    nb_threads: i32,
    sender: Address,
    init_code_hash: FixedBytes<32>,
    pattern: Address,
    mask: Address,
) -> Result<(Address, FixedBytes<32>), Error> {
    let bruteforce_domain = U256::MAX / U256::from(nb_threads);
    let stop_flag = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel();
    let mut threads = vec![];
    for tid in 0..nb_threads {
        let thread_tx = tx.clone();
        let thread_stop_flag = stop_flag.clone();
        let thread = thread::spawn( move || { find_matching_address(
                sender,
                init_code_hash,
                (
                    bruteforce_domain * U256::from(tid),
                    bruteforce_domain * U256::from(tid + 1)
                ),
                pattern,
                mask,
                thread_tx,
                thread_stop_flag
        );
    });
        threads.push(thread);
    }

    if let Ok(result) = rx.recv() {
        stop_flag.store(true, Ordering::Relaxed);

        for thread in threads {
            thread.join().unwrap();
        }

        Ok(result)
    } else {
        Err(Error)
    }
}

