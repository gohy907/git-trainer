#!/bin/bash

cd "$HOME"

OUTPUT="$(cd hello-world && ./main)"

if [ "$OUTPUT" = "Hello, World!" ]; then
	echo "7. Программа выводит \"Hello, World!\"."
    exit 0
else 
    echo "7. Убедитесь, что программа выводит \"Hello, World!\" (найдено: $OUTPUT)."
    exit 1
fi
