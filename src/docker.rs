use bollard::Docker;
use bollard::body_full;

use bollard::errors::Error;
use bollard::models::{ContainerCreateBody, ContainerCreateResponse};
use bollard::query_parameters::{BuildImageOptionsBuilder, CreateContainerOptionsBuilder};
use futures_util::StreamExt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tokio::io;

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

pub fn format_image_name(task_name: &str) -> String {
    format!("git-trainer:{}", task_name)
}

pub async fn create_task_container(task_name: &str) -> Result<ContainerCreateResponse, Error> {
    let docker = Docker::connect_with_socket_defaults()?;
    let create_opts = CreateContainerOptionsBuilder::new().name(task_name).build();

    let config = ContainerCreateBody {
        image: Some(format_image_name(task_name)),
        tty: Some(true),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        open_stdin: Some(true),
        // cmd: Some(vec!["bash".into()]),
        ..Default::default()
    };

    let created = docker.create_container(Some(create_opts), config).await?;

    Ok(created)
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Connection to Docker is not established: {0}")]
    DockerConnection(#[from] bollard::errors::Error),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
}

pub async fn build_task_image(task_name: &str) -> Result<(), BuildError> {
    let docker = Docker::connect_with_socket_defaults()?;

    let name_of_image = format_image_name(task_name);

    let build_options = BuildImageOptionsBuilder::new()
        .dockerfile("src/Dockerfile")
        .t(&name_of_image)
        .build();

    let mut file = File::open(tasks_root().join(task_name).join("src.tar.gz"))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mut build_stream =
        docker.build_image(build_options, None, Some(body_full(contents.into())));

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

use std::process::Command;

pub fn run_interactive(task_name: &str) -> io::Result<()> {
    let image = format!("git-trainer:{}", task_name);

    let status = Command::new("docker")
        .arg("run")
        .arg("-it")
        .arg(&image)
        .arg("bash")
        .status()?;

    if !status.success() {
        eprintln!("docker run exited with status: {status}");
    }

    Ok(())
}
