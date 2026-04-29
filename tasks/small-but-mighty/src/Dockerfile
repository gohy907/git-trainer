FROM git-trainer:base-task-image

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="У вас есть маленькая C++-утилита для кофейни. В ветке feature вы как раз дописываете\nработу с меню и форматирование чека, но коммитить эту незавершённую работу пока рано.\nВ этот момент выясняется, что в main нужно срочно исправить запуск программы без аргументов\nкомандной строки. Ваша задача — не потеряв незавершённые изменения из feature, временно\nпереключиться на main, внести срочный фикс, а затем вернуться в feature и получить обратно изменения, над которыми вы .\n\nИсправленная программа, если не передан аргумент, должна выводить строку \"Usage: ./coffee-counter <drink>\"\n(без кавычек) и возвращать статус-код 1."

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
COPY tasks/small-but-mighty/src/menu.cpp coffee-counter/menu.cpp
COPY tasks/small-but-mighty/src/receipt.cpp coffee-counter/receipt.cpp
