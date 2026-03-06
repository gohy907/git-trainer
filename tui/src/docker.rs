use crate::db::Task;
use bollard::Docker;
use bollard::body_full;
use bollard::container::AttachContainerResults;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::ContainerCreateBody;
use bollard::query_parameters::{
    AttachContainerOptionsBuilder, CreateContainerOptionsBuilder, InspectContainerOptions,
    RemoveContainerOptionsBuilder, ResizeContainerTTYOptionsBuilder, StartContainerOptionsBuilder,
    UploadToContainerOptionsBuilder,
};
use bytes::Bytes;
use tar::Builder;

use futures_util::StreamExt;

fn docker_connect() -> Result<Docker, bollard::errors::Error> {
    Docker::connect_with_socket_defaults()
}

pub async fn ensure_task_container_running(task: &Task) -> Result<(), bollard::errors::Error> {
    let docker = docker_connect()?;

    let exists = match docker
        .inspect_container(
            &task.container_name.clone(),
            None::<InspectContainerOptions>,
        )
        .await
    {
        Ok(_) => true,
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404, ..
        }) => false,
        Err(e) => return Err(e),
    };

    if !exists {
        ensure_task_container_created(task).await?;
    }

    start_container(task).await?;

    Ok(())
}

pub async fn create_task_container(task: &Task) -> Result<String, bollard::errors::Error> {
    let docker = docker_connect()?;
    let create_opts = CreateContainerOptionsBuilder::new()
        .name(&task.container_name)
        .build();

    let config = ContainerCreateBody {
        image: Some(task.image_name.clone()),
        tty: Some(true),
        hostname: Some(task.work_name.clone()),
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

pub async fn ensure_task_container_created(task: &Task) -> Result<String, bollard::errors::Error> {
    let docker = docker_connect()?;

    // eprintln!("{}", &task.work_name);
    match docker
        .inspect_container(&task.container_name, None::<InspectContainerOptions>)
        .await
    {
        Ok(info) => {
            return Ok(info.id.unwrap());
        }
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404, ..
        }) => {}
        Err(e) => {
            return Err(e);
        }
    }
    create_task_container(task).await
}

pub async fn delete_task_container(task: &Task) -> Result<(), bollard::errors::Error> {
    let docker = docker_connect()?;

    let options = RemoveContainerOptionsBuilder::new().build();
    docker
        .remove_container(&task.container_name, Some(options))
        .await
}

pub async fn restart_task(task: &Task) -> Result<(), bollard::errors::Error> {
    match delete_task_container(task).await {
        Ok(_) => {}
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404, ..
        }) => {}
        Err(e) => return Err(e),
    }

    create_task_container(task).await?;
    Ok(())
}

pub async fn start_container(task: &Task) -> Result<(), bollard::errors::Error> {
    let docker = docker_connect()?;
    let start_opts = StartContainerOptionsBuilder::new().build();
    docker
        .start_container(&task.container_name, Some(start_opts))
        .await
}

pub async fn resize_container(
    container_name: String,
    rows: i32,
    cols: i32,
) -> Result<(), bollard::errors::Error> {
    let docker = docker_connect()?;
    let resize_opts = ResizeContainerTTYOptionsBuilder::new()
        .h(rows)
        .w(cols)
        .build();
    docker
        .resize_container_tty(&container_name, resize_opts)
        .await
}

pub async fn attach_container(
    task: &Task,
) -> Result<AttachContainerResults, bollard::errors::Error> {
    let docker = docker_connect()?;
    let attach_opts = AttachContainerOptionsBuilder::new()
        .stdin(true)
        .stdout(true)
        .stderr(true)
        .stream(true)
        .logs(false)
        .build();

    docker
        .attach_container(&task.container_name, Some(attach_opts))
        .await
}

pub struct CmdOutput {
    pub output: String,
    pub exit_code: i64,
}

pub async fn exec_command(task: &Task, cmd: &str) -> Result<CmdOutput, bollard::errors::Error> {
    let docker = docker_connect()?;
    let cmd_string: Vec<&str> = cmd.split_whitespace().collect();

    let exec = docker
        .create_exec(
            &task.container_name,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(cmd_string),
                ..Default::default()
            },
        )
        .await?;

    let mut result = String::new();
    if let StartExecResults::Attached { mut output, .. } = docker.start_exec(&exec.id, None).await?
    {
        while let Some(Ok(msg)) = output.next().await {
            result.push_str(&msg.to_string());
        }
    } else {
        return Err(bollard::errors::Error::DockerContainerWaitError {
            error: "Failed to attach".to_string(),
            code: 0,
        });
    }

    let inspect = docker.inspect_exec(&exec.id).await?;
    let exit_code = inspect.exit_code.unwrap_or(-1);

    Ok(CmdOutput {
        output: result,
        exit_code,
    })
}

pub async fn copy_directory(
    container_name: &str,
    source_dir: &str,
    target_path: &str,
) -> Result<(), bollard::errors::Error> {
    let docker = docker_connect()?;
    let mut tar_data = Vec::new();
    {
        let mut builder = Builder::new(&mut tar_data);

        // Рекурсивно добавляем все файлы
        builder.append_dir_all(".", source_dir)?;
        builder.finish()?;
    }

    let options = UploadToContainerOptionsBuilder::new()
        .path(target_path)
        .build();

    docker
        .upload_to_container(
            container_name,
            Some(options),
            body_full(Bytes::from(tar_data)),
        )
        .await?;

    Ok(())
}
