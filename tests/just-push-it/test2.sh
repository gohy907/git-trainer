#!/bin/bash

REPO_DIR="$HOME/just-push-it"

if [ -d "$REPO_DIR/.git" ] && git -C "$REPO_DIR" status &>/dev/null; then
    echo "2. Git-репозиторий существует."
    exit 0
else
    echo "2. Убедитесь, что в директории just-push-it существует Git-репозиторий."
    exit 1
fi
