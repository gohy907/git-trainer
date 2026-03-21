#!/bin/bash

NEW_DIR="/etc/git-trainer/hello-world"
sudo cp -r "$HOME/hello-world/" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

sudo git reset --hard HEAD &> /dev/null && sudo git clean -fdx &> /dev/null
EXEC_FILES=$(find . -maxdepth 1 -type f -executable)
if [ -n "$EXEC_FILES" ]; then
    sudo rm -rf "$NEW_DIR"
    echo "8. В коммите найдены исполняемые файлы: $EXEC_FILES"
    exit 1
else
    sudo rm -rf "$NEW_DIR"
    echo "8. Исполняемые файлы в коммите не найдены."
    exit 0
fi
