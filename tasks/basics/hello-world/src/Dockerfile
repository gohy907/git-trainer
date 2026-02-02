FROM rust:latest AS builder

WORKDIR /build
COPY Cargo.toml .
COPY Cargo.lock .
COPY tui ./tui/
COPY cli ./cli/
RUN cargo build --release -p cli

FROM debian:bookworm-slim

ARG USERNAME=student
ARG GIT_USERNAME=student
ARG GIT_EMAIL=student@alivetech.com

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

RUN echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen && \
    locale-gen && \
    update-locale LANG=en_US.UTF-8

ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

RUN useradd --create-home --shell /bin/bash "$USERNAME"
RUN adduser "$USERNAME" sudo && \
    echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

COPY --from=builder /build/target/release/cli /usr/local/bin/git-trainer
RUN chown root:root /usr/local/bin/git-trainer && \
    chmod u+s /usr/local/bin/git-trainer

WORKDIR /home/$USERNAME

RUN mkdir -p /etc/git-trainer
RUN mkdir -p /etc/git-trainer/tests
RUN echo -n 0 > /etc/git-trainer/status

USER $USERNAME
RUN git config --global user.name "$GIT_USERNAME" && \
    git config --global user.email "$GIT_EMAIL" && \
    git config --global --add safe.directory /home/$USERNAME

