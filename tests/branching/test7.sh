#!/bin/bash

NEW_DIR="/etc/git-trainer/counting-sort"
sudo cp -r "$HOME/counting-sort" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 
sudo g++ main.cpp

if [[ "$?" = 0 ]]; then
    sudo rm -rf "$NEW_DIR"
    echo "7. Программа компилируется."
    exit 0
else
    sudo rm -rf "$NEW_DIR"
    echo "7. Убедитесь, что программа компилируется."
    exit 1
fi
