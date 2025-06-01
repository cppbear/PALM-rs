# PALM

## Prerequisites

1. Install the Rust toolchain `nightly-2024-07-21`

    ```sh
    rustup install nightly-2024-07-21
    ```

2. Add the required components

    ```sh
    rustup component add --toolchain nightly-2024-07-21 rust-src rustc-dev llvm-tools-preview
    ```

3. Install [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov): `cargo +stable install cargo-llvm-cov --locked`

3. Ensure that the `crate-type` of the crate being analyzed is either "bin" or "lib".

> ⚠️: In order to interact with LLMs, you need to prepare the API key and address of the LLMs yourself.

## Project Structure

This project consists of three sub-projects: brinfo, focxt, and utgen

### brinfo

brinfo is used to extract conditional chain information from the program and save it. For build and usage instructions, refer to [brinfo](brinfo/README.md)

### focxt

focxt is used to extract context information from the program and save it. For build and usage instructions, refer to [focxt](focxt/README.md)

### utgen

utgen generates unit tests by utilizing the conditional chain and context information extracted by brinfo and focxt, in combination with LLMs.
For build and usage instructions, refer to [utgen](utgen/README.md)

<span id="install"></span>

## Installation

Before installation, set `api.json` for `utgen`. You can refer to [utgen](utgen/README.md) for the detailed instruction.

You can use the `install.sh` script to install `cargo brinfo`, `focxt`, and `utgen`.
Make sure the relevant path is in your system's PATH environment variable, typically `$HOME/.cargo/bin`.

## Docker

You can run `docker/docker-build` to build an image if you do not want to setup Rust toolchains in your environment.
You may need to configure mirrors for Ubuntu, crates.io, etc. in the `docker/Dockerfile` to accelerate the build speed.

After build, run `docker/docker-run` to map the `palm` directory into the container, then you can follow [Installation](#install).

## Workflow

First, run `cargo brinfo` and `focxt` to generate the conditional chain and context information.
Then, run `utgen` to generate tests and execute them to collect data.
