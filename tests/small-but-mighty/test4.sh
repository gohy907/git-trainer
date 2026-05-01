#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "4. Произошла системная ошибка, сообщите о ней преподавателю."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "4. В ветке feature остались незакоммиченные изменения."
    exit 0
else
    echo "4. Убедитесь, что после возвращения в feature ваша незавершённая работа снова появилась в рабочем дереве."
    exit 1
fi
