#!/bin/bash

REPO_DIR="$HOME/forget-about-that"

if [ -f "$REPO_DIR/.env" ]; then
    echo "3. Файл .env остался в файловой системе."
    exit 0
else
    echo "3. Убедитесь, что файл .env существует в файловой системе."
    exit 1
fi
