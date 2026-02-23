find tasks/ -name "build.sh" -exec ./{} \;
cargo build --release -p tui && sudo cp target/release/tui /usr/local/bin/.
