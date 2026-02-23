git clone git@github.com:git-trainer-tasks/hello-world src/repo
cd ../..
docker build -f tasks/hello-world/src/Dockerfile -t git-trainer:hello-world .
