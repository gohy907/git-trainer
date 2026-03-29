#!/bin/bash

NEW_DIR="/etc/git-trainer/binary-addition"
sudo cp -r "$HOME/binary-addition" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

g++ main.cpp -o main

if [ "$?" -eq 0 ]; then
    echo "7. Программа компилируется."
    exit 0
else
    echo "7. Убедитесь, что программа компилируется." 
    exit 1
fi
