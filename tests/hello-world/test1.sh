#!/bin/bash

cd "$HOME"

cd hello-world 2>/dev/null

if [ "$?" -eq "0" ]; then
	echo "1. Директория hello-world существует."
	exit 0
else 
	echo "1. Убедитесь, что директория hello-world существует."
	exit 1
fi
