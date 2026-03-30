#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
RUN_DIR="$(mktemp -d /tmp/git-trainer-run.XXXXXX)"
trap 'rm -rf "$TMP_DIR" "$RUN_DIR"' EXIT

EXPECTED_OUTPUT="Usage: ./coffee-counter <drink>"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "8. Произошла системная ошибка, сообщите о ней преподавателю."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

git archive main | tar -x -C "$RUN_DIR"

if ! (cd "$RUN_DIR" && g++ -std=c++17 main.cpp menu.cpp receipt.cpp -o coffee-counter); then
    echo "8. Не удалось собрать программу из ветки main."
    exit 1
fi

OUTPUT="$(cd "$RUN_DIR" && ./coffee-counter 2>&1)"
EXIT_CODE="$?"

if [ "$EXIT_CODE" = "1" ] && [ "$OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "8. Срочный фикс в main корректно обрабатывает запуск без аргументов."
    exit 0
else
    echo "8. Убедитесь, что в main программа без аргументов печатает \"Usage: ./coffee-counter <drink>\" и завершается с кодом 1."
    exit 1
fi
