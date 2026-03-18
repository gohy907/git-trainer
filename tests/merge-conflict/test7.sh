#!/bin/bash

cd "$HOME/binary-addition"

git switch main &>/dev/null
g++ main.cpp &>/dev/null

if [ "$?" -eq 0 ]; then
    echo "7. Программа компилируется."
    exit 0
else
    echo "7. Убедитесь, что программа компилируется." 
    exit 1
fi
