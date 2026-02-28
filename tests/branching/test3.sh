#!/bin/bash

cd "$HOME"

cd counting-sort &>/dev/null

git switch print_vector &>/dev/null

if [ "$?" -eq 0 ]; then
	echo "3. Ветка print_vector существует." 
	exit 0
else 
	echo "3. Убедитесь, что ветка print_vector существует." 
	exit 1
fi
