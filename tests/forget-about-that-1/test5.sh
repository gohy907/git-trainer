#!/bin/bash

REPO_DIR="$HOME/forget-about-that"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "5. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

if [ ! -f .gitignore ]; then
    echo "5. Убедитесь, что файл .gitignore существует и содержит .env."
    exit 1
fi

NORMALIZED_GITIGNORE="$(sed 's/\r$//' .gitignore)"

echo "$NORMALIZED_GITIGNORE" | grep -qxF '.env' >/dev/null 2>&1
HAS_EXACT_LINE="$?"

git check-ignore -q .env
IS_IGNORED="$?"

if [ "$HAS_EXACT_LINE" -eq 0 ] && [ "$IS_IGNORED" -eq 0 ]; then
    echo "5. Файл .env добавлен в .gitignore и корректно игнорируется."
    exit 0
else
    echo "5. Убедитесь, что .env добавлен именно в .gitignore и реально игнорируется Git."
    exit 1
fi
