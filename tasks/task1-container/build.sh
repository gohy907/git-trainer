#!/bin/sh

git_username=$(git config --global user.name)
git_email=$(git config --global user.email)

git clone git@github.com:git-trainer-tasks/task1
cp -r ../../git-trainer-in-container .
docker build --build-arg USERNAME=$USER --build-arg GIT_USERNAME=$git_username --build-arg GIT_EMAIL=$git_email --rm -t task1:latest .
rm -rf task1
rm -rf git-trainer-in-container/
docker run -it --name task1 task1:latest 
