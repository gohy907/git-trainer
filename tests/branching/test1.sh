#!/bin/bash

cd "$HOME"

cd counting-sort 2>/dev/null

if [ "$?" -eq "0" ]; then
	echo "1. Директория counting-sort существует."
	exit 0
else 
	echo "1. Убедитесь, что директория counting-sort существует."
	exit 1
fi
