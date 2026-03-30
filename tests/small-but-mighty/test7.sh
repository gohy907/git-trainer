#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_COMMIT_COUNT="2"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "7. Произошла системная ошибка, сообщите о ней преподавателю."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

ACTUAL_COMMIT_COUNT="$(git rev-list --count main 2>/dev/null)"
CHANGED_FILES="$(git diff --name-only main^ main 2>/dev/null)"
FIXED_MAIN_CONTENT="$(git show main:main.cpp 2>/dev/null)"
PREVIOUS_MAIN_CONTENT="$(git show main^:main.cpp 2>/dev/null)"

if [ "$ACTUAL_COMMIT_COUNT" = "$EXPECTED_COMMIT_COUNT" ] \
   && [ "$CHANGED_FILES" = "main.cpp" ] \
   && printf '%s' "$FIXED_MAIN_CONTENT" | grep -Fq 'Usage: ./coffee-counter <drink>' \
   && ! printf '%s' "$PREVIOUS_MAIN_CONTENT" | grep -Fq 'Usage: ./coffee-counter <drink>'; then
    echo "7. Ветка main получила ровно один срочный фикс."
    exit 0
else
    echo "7. Убедитесь, что в main появился ровно один новый коммит."
    exit 1
fi
