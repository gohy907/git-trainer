#!/bin/bash

NEW_DIR="/etc/git-trainer/binary-addition"
sudo cp -r "$HOME/binary-addition" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

git merge-base --is-ancestor remotes/origin/slim-s/feats main &> /dev/null
CMD1="$(echo $?)"
git merge-base --is-ancestor remotes/origin/gohy907/feats main &> /dev/null
CMD2="$(echo $?)"

if [[ "$CMD1" = 0 && "$CMD2" = 0 ]]; then
    echo "5. Все ветки были соединены в main."
    sudo rm -rf "$NEW_DIR"
    exit 0
else
    echo "5. Убедитесь, что все ветки соединены в main." 
    sudo rm -rf "$NEW_DIR"
    exit 1
fi
