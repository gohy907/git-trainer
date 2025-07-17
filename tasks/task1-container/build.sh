#!/bin/sh

git_username=$(git config --global user.name)
git_email=$(git config --global user.email)

git clone git@github.com:git-trainer-tasks/task1 > /dev/null 2>&1
docker build --build-arg USERNAME=$USER --build-arg GIT_USERNAME=$git_username --build-arg GIT_EMAIL=$git_email --rm -t task1:latest . > /dev/null 2>&1
rm -rf task1
docker run --rm -it --name task1 task1:latest 
