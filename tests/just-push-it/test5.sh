#!/bin/bash

REPO_DIR="$HOME/just-push-it"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "5. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1
git reset --hard HEAD &>/dev/null
git clean -fdx &>/dev/null

if grep -qxF '# Просто запушь это' README.md >/dev/null 2>&1 && \
   grep -qxF 'Этот репозиторий содержит небольшую консольную программу на C++.' README.md >/dev/null 2>&1; then
    echo "5. README.md содержит перевод из удалённого репозитория."
    exit 0
else
    echo "5. Убедитесь, что вы забрали удалённый коммит с переводом README.md."
    exit 1
fi
