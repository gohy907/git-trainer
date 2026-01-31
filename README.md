Для запуска проекта в настоящем мегасыром виде, сделайте эти команды:

```bash
cd tasks/basics/hello-world
git clone git@github.com:git-trainer-tasks/hello-world src/repo
tar czf src.tar.gz src
cd src
docker build . -t git-trainer:hello-world
cd ../../../..
cargo run
```
