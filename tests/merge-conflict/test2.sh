#!/bin/bash

cd "$HOME"

cd binary-addition && git status &>/dev/null

if [ "$?" -eq 0 ]; then
	echo "2. Git-репозиторий существует."
	exit 0
else 
	echo "2. Убедитесь, что в директории binary-addition существует Git-репозиторий."
	exit 1
fi
