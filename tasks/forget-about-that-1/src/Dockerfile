FROM git-trainer:base-task-image

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="Ваш друг разрабатывал проект и решил сделать его конфигурируемым. Для этого он начал считывать так называемые .env файлы. Однако он забыл, что подобные файлы могут хранить различные чувствительные данные (пароли, API-ключи и прочее), которые не следует сохранять в репозитории. Ваша задача, чтобы секреты, сохранённые в .env файле навсегда покинули историю репозитория вашего друга. Для этого вам необходимо модифицировать последний коммит, а именно удалить из него .env файл (так, чтобы его не было в репозитории, но был на диске), а также добавить его в .gitignore файл, чтобы никогда больше не раскрыть секретов вашего стартапа. Эти все изменения должны оказаться в уже существующем последнем коммите, вы не должны создавать новых коммитов."

COPY tasks/forget-about-that-1/src/repo forget-about-that
RUN sudo chown -R $USERNAME:$USERNAME forget-about-that

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
RUN git config --global --add safe.directory /home/$USERNAME/forget-about-that
RUN git config --global user.name "$GIT_USERNAME" && \
    git config --global user.email "$GIT_EMAIL" && \
    git config --global --add safe.directory /home/$USERNAME
RUN cd forget-about-that && git switch main
