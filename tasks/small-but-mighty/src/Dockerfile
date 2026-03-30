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
RUN cd coffee-counter && git switch feature
COPY tasks/small-but-mighty/src/menu.py coffee-counter/menu.py
COPY tasks/small-but-mighty/src/receipt.py coffee-counter/receipt.py
