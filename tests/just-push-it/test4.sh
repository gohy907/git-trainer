#!/bin/bash

REPO_DIR="$HOME/just-push-it"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_OUTPUT=$'max(7, 3) = 7\nmax(-4, 2) = 2'

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "4. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1
git reset --hard HEAD &>/dev/null
git clean -fdx &>/dev/null

if ! g++ -std=c++17 -Wall -Wextra -pedantic main.cpp -o just-push-it &>/dev/null; then
    echo "4. Не удалось собрать программу для проверки результата."
    exit 1
fi

ACTUAL_OUTPUT="$(./just-push-it)"

if [ "$ACTUAL_OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "4. Программа работает корректно."
    exit 0
else
    echo "4. Убедитесь, что max_of_two реализована правильно (получено: $ACTUAL_OUTPUT)."
    exit 1
fi
