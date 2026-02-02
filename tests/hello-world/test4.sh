#!/bin/bash

cd "$HOME"

COMMIT_NAME=$(cd hello-world && git log --oneline --format=%s)

if [ "$COMMIT_NAME" = "Initial commit" ]; then 
	echo "4. Коммит имеет название \"Initial commit\"."
	exit 0
else 
    echo "4. Убедитесь, что коммит имеет название \"Initial commit\" (найдено: $COMMIT_NAME)."
	exit 1
fi

