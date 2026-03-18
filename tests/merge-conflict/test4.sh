#!/bin/bash

NEW_DIR="/etc/git-trainer/binary-addition"
sudo cp -r "$HOME/binary-addition" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

OUT="$(git rev-parse slim-s/feats 2>/dev/null)"

if [[ "$?" = 128 || "$OUT" = "4113823c64d59a05f8de55ff36f74230e033bc68" ]]; then
	echo "4. В ветке slim-s/feats нет новых коммитов." 
    sudo rm -rf "$NEW_DIR"
	exit 0
else 
	echo "4. Убедитесь, что в ветке slim-s/feats нет новых коммитов."
    sudo rm -rf "$NEW_DIR"
	exit 1
fi
