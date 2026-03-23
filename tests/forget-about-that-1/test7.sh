#!/bin/bash

REPO_DIR="$HOME/forget-about-that"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "7. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

if git log HEAD --format=%H -- .env | grep -q .; then
    echo "7. Убедитесь, что .env удалён из истории текущей ветки, а не только из рабочего дерева."
    exit 1
else
    echo "7. Файла .env нет в истории текущей ветки."
    exit 0
fi
