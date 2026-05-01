#!/bin/bash

REPO_DIR="$HOME/just-push-it"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "3. Произошла системная ошибка, сообщите о ней преподавателю."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1
git reset --hard HEAD &>/dev/null
git clean -fdx &>/dev/null

if g++ main.cpp -o just-push-it &>/dev/null; then
    echo "3. Программа компилируется."
    exit 0
else
    echo "3. Убедитесь, что main.cpp компилируется."
    exit 1
fi
