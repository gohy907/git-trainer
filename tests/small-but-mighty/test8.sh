#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
RUN_DIR="$(mktemp -d /tmp/git-trainer-run.XXXXXX)"
trap 'rm -rf "$TMP_DIR" "$RUN_DIR"' EXIT

EXPECTED_OUTPUT="Usage: python3 app.py <drink>"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "8. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

git archive main | tar -x -C "$RUN_DIR"

OUTPUT="$(cd "$RUN_DIR" && python3 app.py 2>&1)"
EXIT_CODE="$?"

if [ "$EXIT_CODE" = "1" ] && [ "$OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "8. Срочный фикс в main корректно обрабатывает запуск без аргументов."
    exit 0
else
    echo "8. Убедитесь, что в main программа без аргументов печатает \"Usage: python3 app.py <drink>\" и завершается с кодом 1."
    exit 1
fi
