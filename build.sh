#! /bin/bash

cargo build --target wasm32-unknown-unknown --release &&
wasm-bindgen --out-dir .public --target web target/wasm32-unknown-unknown/release/winit-web-touch-event-test.wasm
