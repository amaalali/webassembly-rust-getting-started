# Browser's Webassembly API

Simple playing around with browser's built in WebAssembly APIs following instructions from Egghead's _Using WebAssembly with Rust_.


## Current Instructions

Compile and output:

`rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib ./src/<file-name> -o <file-name>.big.wasm`


## Old Instructions
Compile Rust code: `cargo build --target wasm32-unknown-unknown --release`
Process using Wasm-gc: `wasm-gc target/wasm32-unknown-unknown/release/utils.wasm -o utils.gc.wasm`