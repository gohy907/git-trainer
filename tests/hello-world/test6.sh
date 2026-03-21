#!/bin/bash

NEW_DIR="/etc/git-trainer/hello-world"
sudo cp -r "$HOME/hello-world" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

sudo g++ -o main main.cpp

if [ "$?" -eq "0" ]; then 
    echo "6. Файл main.cpp компилируется."
    sudo rm -rf "$NEW_DIR"
    exit 0
else 
    echo "6. Убедитесь, что файл main.cpp компилируется"
    sudo rm -rf "$NEW_DIR"
    exit 1
fi

