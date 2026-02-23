# Вот так стоит добавлять репозиторий из организации git-trainer-tasks для создания образа задания.
# В случае задания hello-world стартового репозитория не должно быть, поэтому тут ничего и не клонируется
# git clone git@github.com:git-trainer-tasks/hello-world tasks/hello-world/src/repo 
docker build -f tasks/hello-world/src/Dockerfile -t git-trainer:hello-world .
