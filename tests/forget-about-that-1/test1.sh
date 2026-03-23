#!/bin/bash

REPO_DIR="$HOME/forget-about-that"

if [ -d "$REPO_DIR" ]; then
    echo "1. Директория forget-about-that существует."
    exit 0
else
    echo "1. Убедитесь, что директория forget-about-that существует."
    exit 1
fi
