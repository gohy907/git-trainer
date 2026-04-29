#!/bin/bash

REPO_DIR="$HOME/just-push-it"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_COMMIT_COUNT="3"
EXPECTED_ROOT_MESSAGE="chore: bootstrap just-push-it project"
EXPECTED_REMOTE_MESSAGE="docs: перевести README на русский язык"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "7. Произошла системная ошибка, сообщите о ней преподавателю."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1
git reset --hard HEAD &>/dev/null
git clean -fdx &>/dev/null

ACTUAL_COMMIT_COUNT="$(git rev-list --count HEAD 2>/dev/null)"
HEAD_PARENT_COUNT="$(git rev-list --parents -n 1 HEAD 2>/dev/null | awk '{print NF - 1}')"
PARENT_MESSAGE="$(git log -1 --format=%s HEAD^ 2>/dev/null)"
ROOT_MESSAGE="$(git log -1 --format=%s HEAD^^ 2>/dev/null)"

if [ "$ACTUAL_COMMIT_COUNT" = "$EXPECTED_COMMIT_COUNT" ] && \
   [ "$HEAD_PARENT_COUNT" = "1" ] && \
   [ "$PARENT_MESSAGE" = "$EXPECTED_REMOTE_MESSAGE" ] && \
   [ "$ROOT_MESSAGE" = "$EXPECTED_ROOT_MESSAGE" ]; then
    echo "7. История линейная: сначала базовый коммит, затем удалённый коммит, затем ваш коммит."
    exit 0
elif [ "$ACTUAL_COMMIT_COUNT" != "$EXPECTED_COMMIT_COUNT" ]; then
    echo "7. Убедитесь, что вы сделали ровно один коммит."
    exit 1
else
    echo "7. Убедитесь, что вы подтянули удалённый коммит через rebase и не создали merge-коммит."
    exit 1
fi
