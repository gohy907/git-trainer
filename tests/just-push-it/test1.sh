#!/bin/bash

REPO_DIR="$HOME/just-push-it"

if [ -d "$REPO_DIR" ]; then
    echo "1. Директория just-push-it существует."
    exit 0
else
    echo "1. Убедитесь, что директория just-push-it существует."
    exit 1
fi
