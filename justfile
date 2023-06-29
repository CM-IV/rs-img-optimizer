build:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

run:
    cargo run