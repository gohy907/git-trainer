#!/bin/bash

cd "$HOME"

cd counting-sort &>/dev/null

CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
MAIN_TIP=$(git rev-parse main)
MERGE_BASE=$(git merge-base HEAD main)

if [ "$MAIN_TIP" = "$MERGE_BASE" ]; then
    echo "Ветка $CURRENT_BRANCH создана от main"
    exit 0
else
    echo "Ветка $CURRENT_BRANCH НЕ создана от main (общий предок: $MERGE_BASE)"
    exit 1
fi
