#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_COMMIT_COUNT="2"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "6. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

ACTUAL_COMMIT_COUNT="$(git rev-list --count feature 2>/dev/null)"
ACTUAL_PARENT_HASH="$(git rev-parse feature^ 2>/dev/null)"
EXPECTED_PARENT_HASH="$(git rev-parse main^ 2>/dev/null)"

if [ "$ACTUAL_COMMIT_COUNT" = "$EXPECTED_COMMIT_COUNT" ] \
   && [ "$ACTUAL_PARENT_HASH" = "$EXPECTED_PARENT_HASH" ]; then
    echo "6. Ветка feature не получила новых коммитов."
    exit 0
else
    echo "6. Убедитесь, что незавершённая работа в feature так и осталась без нового коммита."
    exit 1
fi
