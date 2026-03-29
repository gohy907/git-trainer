FROM git-trainer:base-task-image

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="В этом репозитории вы с другом пишете алгоритм сортировки подсчётом.\nВаш друг сделал отдельную ветку \"origin/counting_sort\", где имплементировал алгоритм, а вам досталась задача написать функцию вывода вектора на экран в другой ветке.\nНапишите эту функцию в ветке \"print_vector\" и объедините обе ваших ветки с главной веткой main."

COPY tasks/branching/src/repo counting-sort
RUN sudo chown -R $USERNAME:$USERNAME counting-sort/

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
RUN git config --global --add safe.directory /home/$USERNAME/counting-sort
RUN cd counting-sort && git switch main
