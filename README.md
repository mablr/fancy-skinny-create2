# fancy-skinny-create2

`fancy-skinny-create2` is a Rust library for finding a `salt` that generates an Ethereum contract address matching a specific pattern using the CREATE2 opcode.

## Features

- **Multi-threaded**: Brute-forces across multiple threads to find the appropriate salt quickly.
- **Flexible pattern matching**: Allows specifying a target pattern and mask for the desired address.
- **Efficient**: Stops as soon as the correct salt is found, ensuring the process is optimized.

## How It Works

This library helps you find a salt for a contract address that matches a given pattern by brute-forcing different salt values. The core function, `salt_bruteforce`, divides the search space across multiple threads and attempts to find a match using the CREATE2 hash generation process.

## Example

See the example in the `examples/fancy_zero.rs` file.

To run the example:

```bash
cargo run --example fancy_zero
```

## Usage
```rust
pub fn salt_bruteforce(
    nb_threads: i32,
    sender: Address,
    init_code_hash: FixedBytes<32>,
    pattern: Address,
    mask: Address,
) -> Result<(Address, FixedBytes<32>), Error>
```

This function attempts to brute-force a salt that, when combined with the provided sender, init code hash, pattern, and mask, produces a matching contract address. It uses multiple threads to speed up the process.

- **Arguments**:
  - `nb_threads`: The number of threads to use for brute-forcing.
  - `sender`: The Ethereum address of the contract deployer.
  - `init_code_hash`: The keccak256 hash of the contract's initialization code.
  - `pattern`: The desired pattern for the resulting contract address.
  - `mask`: The bitmask used to match the pattern.

- **Returns**:
  - `Ok((Address, FixedBytes<32>))`: On success, returns the matching address and the corresponding salt.
  - `Err(Error)`: On failure, returns an error.

## Contributing

Contributions are welcome! Please open issues or submit pull requests to improve the library.

## License

This project is licensed under the MIT License.
