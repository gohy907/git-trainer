#!/bin/bash

REPO_DIR="$HOME/forget-about-that"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "4. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

if git ls-files --error-unmatch .env >/dev/null 2>&1; then
    echo "4. Убедитесь, что .env больше не отслеживается Git."
    exit 1
else
    echo "4. Файл .env больше не отслеживается Git."
    exit 0
fi
