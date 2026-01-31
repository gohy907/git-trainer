FROM debian:bookworm-slim

ARG USERNAME=aboba
ARG GIT_USERNAME=aboba
ARG GIT_EMAIL=aboba@aboba.com

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git \
    neovim \
    nano \
    locales \
    gcc \
    && rm -rf /var/lib/apt/lists/*

RUN echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen && \
    locale-gen && \
    update-locale LANG=en_US.UTF-8

ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

RUN useradd --create-home --shell /bin/bash "$USERNAME"

WORKDIR /home/$USERNAME

RUN git config --global user.name "$GIT_USERNAME" && \
    git config --global user.email "$GIT_EMAIL" && \
    git config --global --add safe.directory /home/$USERNAME

USER $USERNAME
