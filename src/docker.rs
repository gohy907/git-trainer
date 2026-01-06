use bollard::body_full;
use bollard::{API_DEFAULT_VERSION, Docker};
use std::process::Command;

use crate::app::Task;
use bollard::models::ContainerCreateBody;
use bollard::query_parameters::{
    BuildImageOptionsBuilder, CreateContainerOptionsBuilder, InspectContainerOptions,
    RemoveContainerOptionsBuilder,
};
use futures_util::StreamExt;
use nix::unistd::getuid;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Connection to Docker is not established: {0}")]
    DockerConnection(#[from] bollard::errors::Error),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    #[error("Env error: {0}")]
    EnvError(#[from] env::VarError),
}

fn tasks_root() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        PathBuf::from("tasks")
    }

    #[cfg(not(debug_assertions))]
    {
        PathBuf::from("/etc/git-trainer/tasks")
    }
}

fn docker_connect() -> Result<Docker, bollard::errors::Error> {
    let uid = getuid().as_raw();
    let socket = format!("/run/user/{uid}/docker.sock");

    Docker::connect_with_unix(&socket, 120, API_DEFAULT_VERSION)
}

pub fn format_image_name(task_name: &str) -> String {
    format!("git-trainer:{}", task_name)
}

pub async fn create_task_container(task: &Task) -> Result<String, bollard::errors::Error> {
    let docker = docker_connect()?;

    println!("{}", &task.work_name);
    match docker
        .inspect_container(&task.work_name, None::<InspectContainerOptions>)
        .await
    {
        Ok(info) => {
            return Ok(info.id.unwrap());
        }
        Err(bollard::errors::Error::DockerResponseServerError { status_code, .. })
            if status_code == 404 => {}
        Err(e) => {
            return Err(e.into());
        }
    }

    println!("{}", &task.work_name);
    let create_opts = CreateContainerOptionsBuilder::new()
        .name(&task.work_name)
        .build();

    let config = ContainerCreateBody {
        image: Some(format_image_name(&task.work_name)),
        tty: Some(true),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        open_stdin: Some(true),
        // cmd: Some(vec!["bash".into()]),
        ..Default::default()
    };

    let created = docker.create_container(Some(create_opts), config).await?;

    Ok(created.id)
}

pub async fn build_task_image(task: &Task) -> Result<(), BuildError> {
    let docker = docker_connect()?;

    let name_of_image = format_image_name(&task.work_name);

    let build_options = BuildImageOptionsBuilder::new()
        .dockerfile("src/Dockerfile")
        .t(&name_of_image)
        .build();

    let mut file = File::open(
        tasks_root()
            .join(&task.dir)
            .join(&task.work_name)
            .join("src.tar.gz"),
    )?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mut build_stream =
        docker.build_image(build_options, None, Some(body_full(contents.into())));
    println!("asdasdasd");
    while let Some(result) = build_stream.next().await {
        match result {
            Ok(output) => {
                if let Some(stream) = output.stream {
                    print!("{}", stream);
                }
            }
            Err(e) => eprintln!("Build error: {}", e),
        }
    }

    Ok(())
}

pub async fn delete_task_container(task: &Task) -> Result<(), bollard::errors::Error> {
    let docker = docker_connect()?;

    let options = RemoveContainerOptionsBuilder::new().build();
    docker
        .remove_container(&task.work_name, Some(options))
        .await?;
    Ok(())
}

pub async fn restart_task(task: &Task) -> Result<(), bollard::errors::Error> {
    delete_task_container(task).await?;
    create_task_container(task).await?;
    Ok(())
}

pub fn run_interactive(task: &Task) -> io::Result<()> {
    let status = Command::new("docker")
        .arg("start")
        .arg("-ai")
        .arg(&task.work_name)
        .status()?;

    if !status.success() {
        eprintln!("docker run exited with status: {status}");
    }

    Ok(())
}
