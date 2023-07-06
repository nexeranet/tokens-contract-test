#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p release/
cp target/wasm32-unknown-unknown/release/tokens_contract_test.wasm ./release/
