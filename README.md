# ZKP-Based Voting System

This project implements a simple Zero-Knowledge Proof (ZKP) based voting system using Rust and the ZoKrates library. It demonstrates how ZKPs can be used to create a voting system where voters can prove their eligibility without revealing their secret information.

## Features

- Voter registration with secret generation
- ZKP-based vote casting
- Vote counting
- Prevention of double voting
- Public voter hash verification

## Requirements

- Rust 1.51 or later
- Cargo

## Dependencies

- zokrates_core
- zokrates_field
- rand
- sha2
- hex
- thiserror
- serde_json

## Installation

1. Clone this repository:
   git clone https://github.com/elizabeth269/VOTING_TESTING.git
   cd voting_testing

2. Build the project:
   cargo build

## Usage

To run the example voting scenario:
cargo run

## Testing

To run the unit tests: cargo test

## Project Structure

- `src/lib.rs`: Contains the core implementation of the voting system.
- `src/main.rs`: Provides an example usage of the voting system.
- `src/tests.rs`: Contains unit tests for the voting system.

## Limitations and Future Improvements

This is a simplified demonstration and should not be used for actual voting systems without significant enhancements. Future improvements could include:

- Distributed key generation and threshold cryptography
- More sophisticated ZKP circuits for complex voting rules
- Improved mechanisms to prevent double-voting
- Secure storage and transmission of data
- Integration with a blockchain for public verifiability

## License

This project is licensed under the MIT License - see the LICENSE file for details.
