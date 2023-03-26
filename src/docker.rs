use std::{env, str};
use anyhow::{Result, Error};
use docker_api::{conn::TtyChunk, opts::ContainerCreateOpts, Container, Docker};
use log::{info, error, debug};
use futures::StreamExt;

#[cfg(unix)]
const DEFAULT_API_URL: &str = "unix:///var/run/docker.sock";

#[cfg(not(unix))]
const DEFAULT_API_URL: &str = "tcp://127.0.0.1:8080";

pub struct Controller {
    pub api: Docker,
}

impl Controller {
    pub fn new() -> Result<Controller> {
        Ok(Controller {
            api: Docker::new(env::var("DOCKER_API_URL").unwrap_or(DEFAULT_API_URL.to_string()))?,
        })
    }

    pub async fn run(&self, name: &str, image: &str, cmd: Vec<&str>, user: &str, volumes: Option<Vec<&str>>) -> Result<()> {
        info!("run: {} {}", image, cmd.clone().join(" "));
        debug!("mounting volumes: {}", volumes.clone().unwrap_or(vec![]).join(", "));
        let opts = ContainerCreateOpts::builder()
            .image(image)
            .name(name)
            .user(user)
            .volumes(volumes.unwrap_or(vec![]))
            .attach_stdin(true)
            .attach_stderr(true)
            .command(cmd)
            .build();
        let container = self.api.containers().create(&opts).await?;
        container.start().await?;
        let tty = container.attach().await?;
        let (mut reader, _writer) = tty.split();
        while let Some(result) = reader.next().await {
            match result {
                Ok(chunk) => log_tty(name, chunk),
                Err(e) => error!("error reading docker log stream: {}", e)
            }
        }
        container.delete().await?;
        Ok(())
    }
}

pub fn log_tty(name: &str, chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => {
            info!("[{}] {}", name, str::from_utf8(&bytes).unwrap_or_default().trim_end())
        }
        TtyChunk::StdErr(bytes) => {
            info!("[{}] {}", name, str::from_utf8(&bytes).unwrap_or_default().trim_end())
        }
        TtyChunk::StdIn(_) => unreachable!(),
    }
}
