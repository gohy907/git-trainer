# Запустить git-trainer
run:
    cargo build -p tui

build-images:
    find -mindepth 2 -name "justfile" -exec just --justfile {} --working-directory=. default \;
