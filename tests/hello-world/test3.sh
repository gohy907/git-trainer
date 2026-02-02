#!/bin/bash

cd "$HOME"

COMMIT_COUNT="$(cd hello-world && git log --oneline 2>/dev/null | wc -l )"
if [ "$COMMIT_COUNT" -eq "1" ]; then
	echo "3. В репозитории ровно один коммит."
	exit 0
else 
    echo "3. Убедитесь, что в репозитории ровно один коммит (найдено: $COMMIT_COUNT)."
	exit 1
fi
