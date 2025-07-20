FROM golang:1.23-alpine as builder

WORKDIR /git-trainer

COPY git-trainer-in-container/. .
COPY ./description.json .
RUN go build -o ./git-trainer .

FROM ubuntu:22.04

ARG USERNAME=user 
ARG GIT_USERNAME=user 
ARG GIT_EMAIL=user@aboba.com
RUN useradd --create-home --shell /bin/bash $USERNAME && apt-get update \
    && apt-get install -y git neovim nano

RUN mkdir /git-trainer

USER $USERNAME
WORKDIR /home/$USERNAME/task1
RUN git config --global user.name $GIT_USERNAME \
    && git config --global user.email $GIT_EMAIL \
    && git config --global --add safe.directory /home/$USERNAME/task1

COPY --from=builder /git-trainer/git-trainer /git-trainer/git-trainer
COPY --from=builder /git-trainer/description.json /git-trainer/description.json
RUN echo "export PATH=$PATH:/git-trainer/" >> ~/.bashrc
