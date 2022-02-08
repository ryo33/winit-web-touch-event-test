#! /bin/bash

cargo build --target wasm32-unknown-unknown --release &&
~/wasm-bindgen0.2.79/bin/wasm-bindgen --out-dir .public --target web target/wasm32-unknown-unknown/release/winit-web-touch-event-test.wasm
