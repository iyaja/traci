#!/bin/sh
cargo run --release
python src/superresolution/superresolution.py
