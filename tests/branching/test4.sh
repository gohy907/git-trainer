#!/bin/bash

cd "$HOME"

cd counting-sort &>/dev/null

git switch print_vector &>/dev/null

COMMIT_COUNT="$(git log --oneline 2>/dev/null | wc -l )"
if [ "$COMMIT_COUNT" -ge "2" ]; then
	echo "4. В новой ветке есть новые коммиты."
	exit 0
else 
	echo "4. Убедитесь, что в новой ветке есть новые коммиты."
	exit 1
fi
