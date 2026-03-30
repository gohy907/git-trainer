#!/bin/bash

REPO_DIR="$HOME/just-push-it"

if [ -z "$(git -C "$REPO_DIR" status --short 2>/dev/null)" ]; then
    echo "8. Рабочее дерево чистое."
    exit 0
else
    echo "8. Убедитесь, что после коммита, rebase и push у вас не осталось незакоммиченных изменений."
    exit 1
fi
