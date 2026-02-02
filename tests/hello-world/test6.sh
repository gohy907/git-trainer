#!/bin/bash

cd "$HOME"

cd hello-world && gcc -o main main.c 

if [ "$?" -eq "0" ]; then 
    echo "6. Файл main.c компилируется."
    exit 0
else 
    echo "6. Убедитесь, что файл main.c компилируется"
    exit 1
fi

