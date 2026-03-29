FROM git-trainer:base-task-image

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="Вы продолжаете делать проекты вместе с другом в отдельных ветках, но на этот раз просто так слить обе ваших ветки в main почему-то не получится — мешает Merge-конфликт.\nВаша задача — не создавая новых коммитов в ваших дополнительных ветках, слить их в main и решить этот конфликт."

COPY tasks/merge-conflict/src/repo binary-addition 
RUN sudo chown -R $USERNAME:$USERNAME binary-addition 

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
RUN git config --global --add safe.directory /home/$USERNAME/binary-addition
RUN cd binary-addition && git switch main
