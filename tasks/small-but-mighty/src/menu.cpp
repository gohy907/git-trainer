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
