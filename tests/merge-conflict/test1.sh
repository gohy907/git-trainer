#!/bin/bash

cd "$HOME"

cd binary-addition > /dev/null

if [ "$?" -eq 0 ]; then
	echo "1. Директория binary-addition существует." 
	exit 0
else 
	echo "1. Убедитесь, что директория binary-addition существует."
	exit 1
fi
