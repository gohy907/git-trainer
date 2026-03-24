FROM git-trainer:base-task-image

RUN apt-get update && apt-get install -y --no-install-recommends python3

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="У вас есть маленькая Python-утилита для кофейни. В ветке feature вы как раз дописываете\nработу с меню и форматирование чека, но коммитить эту незавершённую работу пока рано.\nВ этот момент выясняется, что в main нужно срочно исправить запуск программы без аргументов\nкомандной строки. Ваша задача — не потеряв незавершённые изменения из feature, временно\nпереключиться на main, внести срочный фикс, а затем вернуться в feature.\n\nИсправленная программа, если не передан аргумент, должна выводить строку \"Usage: python3 app.py <drink>\"\n(без кавычек) и возвращать статус-код 1."

COPY tasks/small-but-mighty/src/repo coffee-counter
RUN sudo chown -R $USERNAME:$USERNAME coffee-counter

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
RUN git config --global --add safe.directory /home/$USERNAME/coffee-counter
RUN git config --global user.name "$GIT_USERNAME" && \
    git config --global user.email "$GIT_EMAIL" && \
    git config --global --add safe.directory /home/$USERNAME
RUN cd coffee-counter && git switch feature && \
    printf '%s\n' \
    'MENU = {' \
    '    "espresso": 150,' \
    '    "latte": 220,' \
    '    "tea": 110,' \
    '}' \
    '' \
    'ALIASES = {' \
    '    "expresso": "espresso",' \
    '}' \
    '' \
    '' \
    'def normalize_drink_name(drink_name):' \
    '    normalized_name = drink_name.strip().lower()' \
    '    return ALIASES.get(normalized_name, normalized_name)' \
    '' \
    '' \
    'def get_price(drink_name):' \
    '    normalized_name = normalize_drink_name(drink_name)' \
    '    return MENU[normalized_name]' > menu.py && \
    printf '%s\n' \
    'def format_receipt(drink_name, price):' \
    '    display_name = drink_name.strip().title()' \
    '    return f"Drink: {display_name}\nTotal: {price} RUB\nSee you soon!"' > receipt.py
