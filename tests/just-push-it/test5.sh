#!/bin/bash

REPO_DIR="$HOME/just-push-it"
REMOTE_GIT_DIR="/opt/git-trainer/just-push-it-origin.git"

LOCAL_HEAD="$(git -C "$REPO_DIR" rev-parse HEAD 2>/dev/null)"
REMOTE_HEAD="$(git --git-dir="$REMOTE_GIT_DIR" rev-parse main 2>/dev/null)"

if [ -n "$LOCAL_HEAD" ] && [ "$LOCAL_HEAD" = "$REMOTE_HEAD" ]; then
    echo "5. Последний локальный коммит отправлен в origin."
    exit 0
else
    echo "5. Убедитесь, что вы запушили итоговый коммит в origin/main."
    exit 1
fi
