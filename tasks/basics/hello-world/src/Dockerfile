FROM ubuntu:latest

ARG USERNAME=aboba
ARG GIT_USERNAME=aboba
ARG GIT_EMAIL=aboba@aboba.com

RUN apt-get update
RUN apt-get install -y git neovim nano 

RUN useradd --create-home --shell /bin/bash "$USERNAME"

WORKDIR /home/"$USERNAME"

COPY src/repo hello-world
RUN git config --global user.name $GIT_USERNAME \
    && git config --global user.email $GIT_EMAIL \
    && git config --global --add safe.directory hello-world
