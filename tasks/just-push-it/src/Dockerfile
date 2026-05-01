FROM git-trainer:base-task-image

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="Вам нужно реализовать функцию max_of_two в файле main.cpp, сделать один коммит с вашим изменением и запушить его в origin. Всё! Действуйте :)"

COPY tasks/just-push-it/src/repo just-push-it
COPY tasks/just-push-it/src/just-push-it-origin.git /opt/git-trainer/just-push-it-origin.git
RUN sudo chown -R $USERNAME:$USERNAME just-push-it && \
    sudo chown -R $USERNAME:$USERNAME /opt/git-trainer/just-push-it-origin.git

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
RUN git config --global --add safe.directory /home/$USERNAME/just-push-it
RUN git config --global user.name "$GIT_USERNAME" && \
    git config --global user.email "$GIT_EMAIL" && \
    git config --global --add safe.directory /home/$USERNAME
RUN cd just-push-it && \
    git config --global --add safe.directory /opt/git-trainer/just-push-it-origin.git && \
    git remote set-url origin /opt/git-trainer/just-push-it-origin.git && \
    git switch main
