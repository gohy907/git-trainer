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
