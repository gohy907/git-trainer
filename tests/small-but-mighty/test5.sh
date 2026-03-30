#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_MENU_HASH="3a132579f292eb95c22ed47a23e011cebd27a5864f6fede54d3130c474da705e"
EXPECTED_RECEIPT_HASH="3ca18caa54645bdbaa2af7b690c0d289e9bf04719f08aba7c03e5cac6012bfa8"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "5. Произошла системная ошибка, сообщите о ней преподавателю."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

CHANGED_FILES="$(printf '%s\n%s\n' "$(git diff --name-only)" "$(git diff --cached --name-only)" | sed '/^$/d' | sort -u)"
ACTUAL_MENU_HASH="$(sha256sum menu.cpp | awk '{print $1}')"
ACTUAL_RECEIPT_HASH="$(sha256sum receipt.cpp | awk '{print $1}')"

if [ "$CHANGED_FILES" = $'menu.cpp\nreceipt.cpp' ] \
   && [ "$ACTUAL_MENU_HASH" = "$EXPECTED_MENU_HASH" ] \
   && [ "$ACTUAL_RECEIPT_HASH" = "$EXPECTED_RECEIPT_HASH" ]; then
    echo "5. В ветке feature восстановлена именно незавершённая работа над menu.cpp и receipt.cpp."
    exit 0
else
    echo "5. Убедитесь, что в feature вернулись исходные незакоммиченные изменения в menu.cpp и receipt.cpp."
    exit 1
fi
