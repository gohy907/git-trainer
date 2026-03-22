#!/bin/bash

NEW_DIR="/etc/git-trainer/counting-sort"
sudo cp -r "$HOME/counting-sort" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

sudo git switch print_vector &>/dev/null

COMMIT_COUNT="$(git log --oneline 2>/dev/null | wc -l )"
if [ "$COMMIT_COUNT" -ge "2" ]; then
    sudo rm -rf "$NEW_DIR"
	echo "4. В новой ветке есть новые коммиты."
	exit 0
else 
    sudo rm -rf "$NEW_DIR"
	echo "4. Убедитесь, что в новой ветке есть новые коммиты."
	exit 1
fi
