# Plain Token transfer transactions

A Rust actix web api project that returns the list of ERC-20 tokens transferred by an address, with filtering by token contract using [https://api-sepolia.etherscan.io/api].

## Prerequisites

- Rust

## Installation

To install Rust, follow the instructions [here](https://www.rust-lang.org/tools/install).

## Build & Run

First set up .env file with the following content:
```sh
$ cp .env.example .env

To build the API, execute the following commands from the project root:
```sh
$ cargo run
```
