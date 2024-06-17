# Miden client

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/0xPolygonMiden/miden-client/blob/main/LICENSE)
[![test](https://github.com/0xPolygonMiden/miden-client/actions/workflows/test.yml/badge.svg)](https://github.com/0xPolygonMiden/miden-clinet/actions/workflows/test.yml)
[![RUST_VERSION](https://img.shields.io/badge/rustc-1.78+-lightgray.svg)](https://www.rust-lang.org/tools/install)
[![crates.io](https://img.shields.io/crates/v/miden-client)](https://crates.io/crates/miden-client)

This repository contains the Miden client, which provides a way to execute and prove transactions, facilitating the interaction with the Miden rollup.

### Status

The Miden client is still under heavy development and the project can be considered to be in an *alpha* stage. Many features are yet to be implemented and there is a number of limitations which we will lift in the near future.

## Overview

The Miden client currently consists of two components:

- `miden-client` library, which can be used by other project to programmatically interact with the Miden rollup. 
- `miden-client` binary which is a wrapper around the library exposing its functionality via a simple command-line interface (CLI).

The client's main responsibility is to maintain a partial view of the blockchain which allows for locally executing and proving transactions. It keeps a local store of various entities that periodically get updated by syncing with the node.

For more info check:

- [Getting started](https://docs.polygon.technology/miden/miden-base/introduction/get-started/create-account-use-faucet/)
- [CLI Reference](./docs/cli-reference.md#types-of-transaction)
    - [Configuration](./docs/cli-config.md)
- [Online Documentation](https://docs.polygon.technology/miden/miden-client)

## Usage

Before you can use the Miden client, you'll need to make sure you have both
[Rust](https://www.rust-lang.org/tools/install) and sqlite3 installed. Miden
client requires rust version **1.78** or higher.

### Adding miden-client as a dependency

In order to utilize the `miden-client` library, you can add the dependency to your project's `Cargo.toml` file:

````toml
miden-client = { version = "0.3.1" }
````

By default, the library is `no_std` compatible. 

#### Features

- `concurrent`: used to enable concurrency during execution and proof generation.
- `testing`: useful feature that lowers PoW difficulty when enabled, meant to be used during development and not on production. 
- `sqlite`: includes `SqliteStore`, a SQLite implementation of the `Store` trait that can be used as a component of `Client`.
- `async`: enables async traits. Disabled by default.
- `tonic`: includes `TonicRpcClient`, a Tonic client to communicate with Miden node, that can be used as a component of `Client`. 
- `executable`: builds the CLI, based on `SqliteStore` and `TonicRpcClient`.

### Running `miden-client`'s CLI

You can either build from source with:

```bash
cargo build --release
```

Once the binary is built, you can find it on `./target/release/miden`.

Or you can install the CLI from crates-io with:

```bash
cargo install miden-client
```

Note that binary name for the client is just `miden`.

### Makefile

As mentioned before, we use `make` to encapsulate some tasks, such as running lints and tests. You can check out [Makefile](./Makefile) for all available tasks or just run the following command:

```bash
make
```

## Testing

To test the project's code, we provide both unit tests (which can be run with `cargo test`) and integration tests. For more info on integration tests, refer to the [integration testing document](./tests/README.md)

The crate also comes with one feature flag that is used exclusively on tests: 

- `integration`: only used to run integration tests and separate them from unit tests.

## Contributing

Interested in contributing? Check [CONTRIBUTING.md](./CONTRIBUTING.md).

## License
This project is [MIT licensed](./LICENSE).
