#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
CURRENT_BRANCH="$(git -C "$REPO_DIR" branch --show-current 2>/dev/null)"

if [ "$CURRENT_BRANCH" = "feature" ]; then
    echo "3. В конце работы активна ветка feature."
    exit 0
else
    echo "3. Убедитесь, что после срочного фикса вы вернулись в ветку feature."
    exit 1
fi
