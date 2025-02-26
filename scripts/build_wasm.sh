#!/bin/bash


cd "$(dirname "$0")"/.. || { echo "Failed to change directory"; exit 1; }

cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/game_of_life.wasm .

echo "Finished building WASM"