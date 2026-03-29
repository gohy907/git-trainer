FROM rust:latest AS builder

WORKDIR /build
COPY Cargo.toml .
COPY Cargo.lock .
COPY tui ./tui/
COPY cli ./cli/
RUN cargo build --release -p cli

FROM git-trainer:base-task-image

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com
ENV DESCRIPTION="Давайте начнём с чего-нибудь лёгкого.\nСоздайте в папке \"hello-world\" новый Git репозиторий, в котором напишите main.cpp, выводящий на экран строчку \"Hello, World!\".\nПосле этого сделайте ровно один коммит, добавляющий этот файл, с названием \"Initial commit\"."

# Настройки ниже трогать уже не стоит

RUN echo -n $DESCRIPTION > /etc/git-trainer/description
USER $USERNAME
