#!/bin/bash

NEW_DIR="/etc/git-trainer/counting-sort"
sudo cp -r "$HOME/counting-sort" "$NEW_DIR"
git config --global --add safe.directory "$NEW_DIR"
cd "$NEW_DIR" 

sudo git switch print_vector &>/dev/null

if [ "$?" -eq 0 ]; then
    sudo rm -rf "$NEW_DIR"
	echo "3. Ветка print_vector существует." 
	exit 0
else 
    sudo rm -rf "$NEW_DIR"
	echo "3. Убедитесь, что ветка print_vector существует." 
	exit 1
fi
