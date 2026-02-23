FROM rust:latest AS builder

WORKDIR /build
COPY Cargo.toml .
COPY Cargo.lock .
COPY tui ./tui/
COPY cli ./cli/
RUN cargo build --release -p cli

FROM ubuntu:latest

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com

COPY --from=builder /build/target/release/cli /usr/local/bin/git-trainer
RUN chown root:root /usr/local/bin/git-trainer && \
    chmod u+s /usr/local/bin/git-trainer

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    sudo \
    git \
    neovim \
    nano \
    locales \
    gcc \
    libc6-dev \
    && rm -rf /var/lib/apt/lists/*

RUN sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen && \
    locale-gen en_US.UTF-8 && \
    update-locale LANG=en_US.UTF-8

ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

RUN useradd --create-home --shell /bin/bash "$USERNAME"
RUN usermod -aG sudo "$USERNAME" && \
    echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

WORKDIR /home/$USERNAME

RUN mkdir -p /etc/git-trainer
RUN mkdir -p /etc/git-trainer/tests
RUN echo -n 0 > /etc/git-trainer/status

# Вот с этой строчки вы можете изменять этот Dockerfile для создания новых заданий. То, что находится выше, не стоит трогать ради однообразия образов

# Вот так стоит добавлять репозитории в образ. В случае с заданием hello-world никакого репозитория не требуется, поэтому ничего и не копируется
# COPY tasks/hello-world/src/repo hello-world 

ENV DESCRIPTION="Давайте начнём с чего-нибудь лёгкого.\nСоздайте в папке \"hello-world\" новый Git репозиторий, в котором напишите код на C, выводящий на экран строчку \"Hello, World!\".\nПосле этого сделайте ровно один коммит, добавляющий этот код, с названием \"Initial commit\"."

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
RUN git config --global user.name "$GIT_USERNAME" && \
    git config --global user.email "$GIT_EMAIL" && \
    git config --global --add safe.directory /home/$USERNAME




