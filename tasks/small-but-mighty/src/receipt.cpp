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
