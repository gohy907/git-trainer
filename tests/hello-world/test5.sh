#!/bin/bash

cd "$HOME"

FILES_IN_COMMIT=$(cd hello-world && git show --name-only --format="" HEAD)

if [ "$FILES_IN_COMMIT" = "main.c" ]; then
    echo "5. В коммите есть файл main.c."
    exit 0
else
    echo "5. Убедитесь, что в коммите есть файл main.c."
    exit 1
fi
