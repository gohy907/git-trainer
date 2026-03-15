#!/bin/bash

cd "$HOME/counting-sort"
git merge-base --is-ancestor 663c28 main
CMD1="$(echo $?)"
git merge-base --is-ancestor print_vector main 
CMD2="$(echo $?)"

if [[ "$CMD1" = 0 && "$CMD2" = 0 ]]; then
    echo "6. Все ветки были слиты в main."
    exit 0
else
    echo "6. Убедитесь, что все ветки слиты в main." 
    exit 1
fi
