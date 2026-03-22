#!/bin/bash
# Trinity clippy check script
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis
cargo clippy --workspace --manifest-path ./Cargo.toml --all-targets -- -D warnings
