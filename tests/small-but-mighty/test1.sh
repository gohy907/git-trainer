#!/bin/bash

REPO_DIR="$HOME/coffee-counter"

if [ -d "$REPO_DIR" ]; then
    echo "1. Директория coffee-counter существует."
    exit 0
else
    echo "1. Убедитесь, что директория coffee-counter существует."
    exit 1
fi
