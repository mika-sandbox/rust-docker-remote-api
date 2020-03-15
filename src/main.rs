use std::collections::HashMap;

use bollard::container::*;
use bollard::Docker;

use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::connect_with_local_defaults()?;
    let container_options = Some(CreateContainerOptions::<&str> {
        name: "my-new-container",
    });

    let container_config = Config {
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        image: Some("hello-world"),
        ..Default::default()
    };

    let container = &docker
        .create_container(container_options, container_config)
        .await?;

    &docker
        .start_container(&container.id, None::<StartContainerOptions<String>>)
        .await?;

    let wait_options = Some(WaitContainerOptions {
        condition: "not-running",
    });

    &docker.wait_container(&container.id, wait_options);

    let logs_options = Some(LogsOptions {
        follow: true,
        stdout: true,
        stderr: true,
        ..Default::default()
    });

    let stream = &mut docker.logs(&container.id, logs_options);

    // pin_mut!(stream);

    while let Some(value) = &stream.next().await {
        match value {
            Ok(log) => match log {
                LogOutput::StdOut { message } => println!("stdout: {}", message),
                LogOutput::StdErr { message } => eprintln!("stderr: {}", message),
                LogOutput::StdIn { message } => println!("stdin : {}", message),
                LogOutput::Console { message } => println!("console: {}", message),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
