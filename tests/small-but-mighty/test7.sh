#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_PARENT_HASH="810b5d4e97f76353a141e3351ed4c33cd7b9df13"
EXPECTED_COMMIT_COUNT="2"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "7. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

ACTUAL_COMMIT_COUNT="$(git rev-list --count main 2>/dev/null)"
ACTUAL_PARENT_HASH="$(git rev-parse main^ 2>/dev/null)"

if [ "$ACTUAL_COMMIT_COUNT" = "$EXPECTED_COMMIT_COUNT" ] \
   && [ "$ACTUAL_PARENT_HASH" = "$EXPECTED_PARENT_HASH" ]; then
    echo "7. Ветка main получила ровно один срочный фикс."
    exit 0
else
    echo "7. Убедитесь, что в main появился ровно один новый коммит."
    exit 1
fi
