#!/usr/bin/env bash

set -eux

python3 bindgen/ufbx_parser.py -i ufbx/ufbx.h
python3 bindgen/ufbx_ir.py
python3 bindgen/generate_rust.py
cargo build
