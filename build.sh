#!/usr/bin/env bash
set -euo pipefail

if [ -d /home/antigravity/.rustup ]; then export RUSTUP_HOME=/home/antigravity/.rustup; fi
cargo build --target wasm32-unknown-unknown --release
mkdir -p web
cp target/wasm32-unknown-unknown/release/corewar.wasm web/corewar.wasm
echo "Built web/corewar.wasm ($(du -h web/corewar.wasm | cut -f1))"
