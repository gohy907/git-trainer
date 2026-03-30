#!/bin/bash

REPO_DIR="$HOME/coffee-counter"
TMP_DIR="$(mktemp -d /tmp/git-trainer.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

EXPECTED_MENU_CONTENT="$(cat <<'EOF'
#include "menu.h"

#include <algorithm>
#include <cctype>
#include <map>
#include <string>

namespace {

const std::map<std::string, int> kMenu = {
    {"espresso", 150},
    {"latte", 220},
    {"tea", 110},
};

const std::map<std::string, std::string> kAliases = {
    {"expresso", "espresso"},
};

std::string normalizeDrinkName(const std::string& drinkName) {
    const auto first = std::find_if_not(drinkName.begin(), drinkName.end(), [](unsigned char ch) {
        return std::isspace(ch) != 0;
    });
    const auto last = std::find_if_not(drinkName.rbegin(), drinkName.rend(), [](unsigned char ch) {
        return std::isspace(ch) != 0;
    }).base();

    std::string normalized(first, last);
    std::transform(normalized.begin(), normalized.end(), normalized.begin(), [](unsigned char ch) {
        return static_cast<char>(std::tolower(ch));
    });

    const auto alias = kAliases.find(normalized);
    if (alias != kAliases.end()) {
        return alias->second;
    }

    return normalized;
}

}  // namespace

int getPrice(const std::string& drinkName) {
    return kMenu.at(normalizeDrinkName(drinkName));
}

EOF
)"

EXPECTED_RECEIPT_CONTENT="$(cat <<'EOF'
#include "receipt.h"

#include <algorithm>
#include <cctype>
#include <string>

namespace {

std::string trim(const std::string& value) {
    const auto first = std::find_if_not(value.begin(), value.end(), [](unsigned char ch) {
        return std::isspace(ch) != 0;
    });
    const auto last = std::find_if_not(value.rbegin(), value.rend(), [](unsigned char ch) {
        return std::isspace(ch) != 0;
    }).base();

    return std::string(first, last);
}

std::string formatDrinkName(const std::string& drinkName) {
    std::string formatted = trim(drinkName);
    bool start_of_word = true;

    for (char& ch : formatted) {
        const auto current = static_cast<unsigned char>(ch);
        if (std::isspace(current) != 0) {
            start_of_word = true;
            continue;
        }

        ch = start_of_word ? static_cast<char>(std::toupper(current))
                           : static_cast<char>(std::tolower(current));
        start_of_word = false;
    }

    return formatted;
}

}  // namespace

std::string formatReceipt(const std::string& drinkName, int price) {
    return "Drink: " + formatDrinkName(drinkName) + "\nTotal: " + std::to_string(price) +
           " RUB\nSee you soon!";
}

EOF
)"

if ! cp -R "$REPO_DIR" "$TMP_DIR/repo" 2>/dev/null; then
    echo "5. Произошла системная ошибка, сообщите о ней преподавателю."
    exit 1
fi

cd "$TMP_DIR/repo" || exit 1

CHANGED_FILES="$(printf '%s\n%s\n' "$(git diff --name-only)" "$(git diff --cached --name-only)" | sed '/^$/d' | sort -u)"
ACTUAL_MENU_CONTENT="$(cat menu.cpp)"
ACTUAL_RECEIPT_CONTENT="$(cat receipt.cpp)"

if [ "$CHANGED_FILES" = $'menu.cpp\nreceipt.cpp' ] \
   && [ "$ACTUAL_MENU_CONTENT" = "$EXPECTED_MENU_CONTENT" ] \
   && [ "$ACTUAL_RECEIPT_CONTENT" = "$EXPECTED_RECEIPT_CONTENT" ]; then
    echo "5. В ветке feature восстановлена именно незавершённая работа над menu.cpp и receipt.cpp."
    exit 0
else
    echo "5. Убедитесь, что в feature вернулись исходные незакоммиченные изменения в menu.cpp и receipt.cpp."
    exit 1
fi
