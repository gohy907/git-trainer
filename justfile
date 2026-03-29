# Запустить git-trainer
run: build-images
    cargo run -p tui

# Собрать образы заданий
build-images:
    just --justfile tasks/base/justfile --working-directory=.
    find . -mindepth 2 -name "justfile" -exec just --justfile {} --working-directory=. default \;

# Собрать git-trainer для релиза
release: build-images
    cargo build -p tui --release
    sudo mkdir -p /var/lib/git-trainer
    sudo cp -r tests migrations schema.sql /var/lib/git-trainer/
    sudo cp target/release/tui /usr/bin/git-trainer
