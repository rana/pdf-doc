# pdf-doc

Create a writing document and save to PDF with Rust.

A minimal library.

Model a writing document with formatting options.

Slightly modeled after _Google Docs_ or _Microsoft Word_.

Save to `PDF` or `JSON`.

Reads `JSON` documents only. 

## Prerequisites

Install `clang++` for the [skia-safe](https://crates.io/crates/skia-safe) crate dependency.

On Linux
```sh
sudo apt update
sudo apt install -y clang
clang++ --version
```