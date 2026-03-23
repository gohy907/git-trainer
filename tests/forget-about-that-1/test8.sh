#!/bin/bash

REPO_DIR="$HOME/forget-about-that"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_COMMIT_COUNT="3"
EXPECTED_PARENT_HASH="d6086d04bac7a1efd3705bfd513c1db0e82ea721"
EXPECTED_COMMIT_MESSAGE=$'feat: Make app configurable'

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "8. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

ACTUAL_COMMIT_COUNT="$(git rev-list --count HEAD 2>/dev/null)"
ACTUAL_PARENT_HASH="$(git rev-parse HEAD^ 2>/dev/null)"
ACTUAL_COMMIT_MESSAGE="$(git log -1 --format=%B 2>/dev/null)"

if [ "$ACTUAL_COMMIT_COUNT" = "$EXPECTED_COMMIT_COUNT" ] \
   && [ "$ACTUAL_PARENT_HASH" = "$EXPECTED_PARENT_HASH" ] \
   && [ "$ACTUAL_COMMIT_MESSAGE" = "$EXPECTED_COMMIT_MESSAGE" ]; then
    echo "8. Последний коммит был переписан без добавления нового коммита и без изменения текста сообщения."
    exit 0
else
    echo "8. Убедитесь, что вы не создали новый коммит и не изменили текст сообщения последнего коммита."
    exit 1
fi
