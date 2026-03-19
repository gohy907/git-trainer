find tasks/ -name "build.sh" -exec ./{} \;
cargo build --release -p tui 
sudo mkdir -p /var/lib/git-trainer/ 
sudo cp -r tests migrations schema.sql /var/lib/git-trainer/
sudo cp target/release/tui /var/lib/git-trainer/git-trainer
