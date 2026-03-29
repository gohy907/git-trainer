FROM git-trainer:base-task-image

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="Вы разрабатывали проект, внутри которого используете данные, личные пароли, ключи и явки, описанные в .env файле.\nНо в последнем коммите вы случайно добавили этот файл в репозиторий, и фактически оставили все эти чувствительные\nданные на виду у всех пользователей удалённого репозитория.\n\nВаша задача — сохранить ваши секреты в тайне, вам нужно, чтобы .env файл исчез с лица вашего репозитория,\nно оставить его на файловой системе, сделать всё возможное, чтобы такого недоразумения больше не случилось,\nи всё это — путём редактирования последнего коммита."

COPY tasks/forget-about-that-1/src/repo forget-about-that
RUN sudo chown -R $USERNAME:$USERNAME forget-about-that

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
RUN git config --global --add safe.directory /home/$USERNAME/forget-about-that
RUN cd forget-about-that && git switch main
