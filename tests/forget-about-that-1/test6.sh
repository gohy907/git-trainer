#!/bin/bash

REPO_DIR="$HOME/forget-about-that"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "6. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

if git cat-file -e HEAD:.env 2>/dev/null; then
    echo "6. Убедитесь, что в последнем коммите файла .env больше нет."
    exit 1
else
    echo "6. В последнем коммите файла .env нет."
    exit 0
fi
