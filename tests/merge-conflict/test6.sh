#!/bin/bash

cd "$HOME/binary-addition"

git switch main &>/dev/null

grep "<<<<<<<" main.cpp &>/dev/null && grep ">>>>>>>" main.cpp &>/dev/null

if [ "$?" = 1 ]; then
    echo "6. Merge-конфликт решён."
    exit 0
else
    echo "6. Убедитесь, что Merge-конфликт решён."
    exit 1
fi
