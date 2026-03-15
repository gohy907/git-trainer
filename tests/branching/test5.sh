#!/bin/bash

cd "$HOME/counting-sort"

if [ "$(git show-branch --merge-base print_vector remotes/origin/counting_sort)" = "790d2418057a3b0f0af1bd67d8704d1da92c179f" ]; then
	
    echo "5. Ветка print_vector создана от main."
    exit 0
else
    echo "5. Убедитесь, что ветка print_vector создана от main."
    exit 1
fi
