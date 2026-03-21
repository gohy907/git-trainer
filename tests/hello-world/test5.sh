#!/bin/bash

cd "$HOME"

OUTPUT=$(cd hello-world && git show --name-only --format="" HEAD)
OUTPUT_EXPECTED="main.cpp"

if echo "$OUTPUT" | grep -q "$OUTPUT_EXPECTED" ; then
    echo "5. В коммите есть файл main.cpp."
    exit 0
else
    echo "5. Убедитесь, что в коммите есть файл main.cpp."
    exit 1
fi
