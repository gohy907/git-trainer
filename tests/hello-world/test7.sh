#!/bin/bash

NEW_DIR="/etc/git-trainer/hello-world"
sudo cp -r "$HOME/hello-world" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

sudo g++ -o main main.cpp

OUTPUT="$(./main)"
OUTPUT_EXPECTED="Hello, World!"

if echo "$OUTPUT" | grep -iq "^$OUTPUT_EXPECTED" ; then
    echo "7. Программа выводит \"Hello, World!\"."
    exit 0
else 
    echo "7. Убедитесь, что программа выводит \"Hello, World!\" (найдено: $OUTPUT)."
    exit 1
fi
