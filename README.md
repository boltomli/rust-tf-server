# Serving Tensorflow in Rust

## Setup

Get pretrained `mobilenet_v2_1.4_224_frozen.pb` from model zoo. Update file path in code (or rewrite the part to use environment variable).

## Usage

* Server: `cargo run --release`. Reflection is supported.
* Client: any preferred gRPC client. Read image to bytes, encode in base64 to fill the request data.
