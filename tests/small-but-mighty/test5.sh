#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_MENU_CONTENT="$(cat <<'EOF'
MENU = {
    "espresso": 150,
    "latte": 220,
    "tea": 110,
}

ALIASES = {
    "expresso": "espresso",
}


def normalize_drink_name(drink_name):
    normalized_name = drink_name.strip().lower()
    return ALIASES.get(normalized_name, normalized_name)


def get_price(drink_name):
    normalized_name = normalize_drink_name(drink_name)
    return MENU[normalized_name]

EOF
)"

EXPECTED_RECEIPT_CONTENT="$(cat <<'EOF'
def format_receipt(drink_name, price):
    display_name = drink_name.strip().title()
    return f"Drink: {display_name}\nTotal: {price} RUB\nSee you soon!"

EOF
)"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "5. Не удалось подготовить временную копию репозитория для проверки."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

CHANGED_FILES="$(printf '%s\n%s\n' "$(git diff --name-only)" "$(git diff --cached --name-only)" | sed '/^$/d' | sort -u)"
ACTUAL_MENU_CONTENT="$(cat menu.py)"
ACTUAL_RECEIPT_CONTENT="$(cat receipt.py)"

if [ "$CHANGED_FILES" = $'menu.py\nreceipt.py' ] \
   && [ "$ACTUAL_MENU_CONTENT" = "$EXPECTED_MENU_CONTENT" ] \
   && [ "$ACTUAL_RECEIPT_CONTENT" = "$EXPECTED_RECEIPT_CONTENT" ]; then
    echo "5. В ветке feature восстановлена именно незавершённая работа над menu.py и receipt.py."
    exit 0
else
    echo "5. Убедитесь, что в feature вернулись исходные незакоммиченные изменения в menu.py и receipt.py."
    exit 1
fi
