rustup install nightly
rustup override add nightly
cargo +nightly -Zjson-target-spec rustc --release --target x86_64-meirg.json -- --emit=obj