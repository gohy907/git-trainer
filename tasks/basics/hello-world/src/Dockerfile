FROM ubuntu:latest

ARG USERNAME=aboba
ARG GIT_USERNAME=aboba
ARG GIT_EMAIL=aboba@aboba.com

RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    apt-get update && \
    apt-get install -y \
    git \
    neovim \
    nano \
    locales \
    gcc \
    && rm -rf /var/lib/apt/lists/*
RUN echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen
RUN locale-gen
RUN export LANG=en_US.UTF-8

RUN useradd --create-home --shell /bin/bash "$USERNAME"

WORKDIR /home/"$USERNAME"

# COPY repo hello-world
RUN git config --global user.name $GIT_USERNAME \
    && git config --global user.email $GIT_EMAIL \
    && git config --global --add safe.directory /home/"$USERNAME" 
