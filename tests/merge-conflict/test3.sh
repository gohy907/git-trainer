#!/bin/bash

NEW_DIR="/etc/git-trainer/binary-addition"
sudo cp -r "$HOME/binary-addition" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

OUT="$(git rev-parse gohy907/feats 2>/dev/null)"

if [[ "$?" = 128 || "$OUT" = "8e203de56cd1581a1ba8f3b203c81763f3a8d700" ]]; then
	echo "3. В ветке gohy907/feats нет новых коммитов." 
    sudo rm -rf "$NEW_DIR"
	exit 0
else 
	echo "3. Убедитесь, что в ветке gohy907/feats нет новых коммитов."
    sudo rm -rf "$NEW_DIR"
	exit 1
fi
# fi
