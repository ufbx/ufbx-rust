
parse:
    python3 bindgen/ufbx_parser.py -i ufbx/ufbx.h
    python3 bindgen/ufbx_ir.py

generate:
    python3 bindgen/generate_rust.py

build:
    cargo build

test:
    cargo test
