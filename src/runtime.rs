// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Deep Thinking LLC.
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Default)]
pub struct CreateContainerOpts {
    pub name: String,
    pub image: String,
    pub env: Option<Vec<String>>,
    pub ports: Option<Vec<PortBinding>>,
    pub network: Option<String>,
    pub volumes: Option<Vec<VolumeMount>>,
    pub labels: Option<HashMap<String, String>>,
    pub command: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PortBinding {
    pub container_port: u16,
    pub host_port: Option<u16>,
    pub host_ip: Option<String>,
    pub protocol: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ContainerInfo {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: String,
    pub status: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkInfo {
    pub id: String,
    pub name: String,
    pub driver: String,
}

#[derive(Debug, Clone, Default)]
pub struct ContainerStats {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub network_rx: u64,
    pub network_tx: u64,
}

#[derive(Debug, Clone)]
pub struct ImageAuth {
    pub username: String,
    pub password: String,
    pub registry: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RuntimeCapabilities {
    pub supports_image_pull: bool,
    pub supports_registry_auth: bool,
    pub supports_networks: bool,
    pub supports_volumes: bool,
    pub supports_exec: bool,
    pub supports_copy: bool,
    pub supports_logs: bool,
    pub supports_stats: bool,
}

impl RuntimeCapabilities {
    pub const fn docker() -> Self {
        Self {
            supports_image_pull: true,
            supports_registry_auth: true,
            supports_networks: true,
            supports_volumes: true,
            supports_exec: true,
            supports_copy: true,
            supports_logs: true,
            supports_stats: true,
        }
    }

    pub const fn apple_container() -> Self {
        Self {
            supports_image_pull: true,
            supports_registry_auth: false,
            supports_networks: false,
            supports_volumes: false,
            supports_exec: true,
            supports_copy: true,
            supports_logs: true,
            supports_stats: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LogOpts {
    pub follow: bool,
    pub tail: u32,
    pub since: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub message: String,
    pub level: String,
}

#[derive(Error, Debug, Clone)]
pub enum RuntimeError {
    #[error("Failed to connect to runtime: {0}")]
    Connection(String),

    #[error("Failed to pull image {image}:{tag}: {message}")]
    Pull {
        image: String,
        tag: String,
        message: String,
    },

    #[error("Failed to create container: {0}")]
    CreateContainer(String),

    #[error("Failed to start container {id}: {message}")]
    StartContainer { id: String, message: String },

    #[error("Failed to stop container {id}: {message}")]
    StopContainer { id: String, message: String },

    #[error("Failed to pause container {id}: {message}")]
    PauseContainer { id: String, message: String },

    #[error("Failed to resume container {id}: {message}")]
    ResumeContainer { id: String, message: String },

    #[error("Failed to remove container {id}: {message}")]
    RemoveContainer { id: String, message: String },

    #[error("Failed to list containers: {0}")]
    ListContainers(String),

    #[error("Failed to get logs for container {id}: {message}")]
    Logs { id: String, message: String },

    #[error("Image not found: {image}:{tag}")]
    ImageNotFound { image: String, tag: String },

    #[error("Authentication failed for registry: {0}")]
    AuthError(String),

    #[error("Network operation failed: {0}")]
    Network(String),

    #[error("Volume operation failed: {0}")]
    Volume(String),

    #[error("Failed to execute command in container {id}: {message}")]
    Exec { id: String, message: String },

    #[error("Failed to get stats for container {id}: {message}")]
    Stats { id: String, message: String },

    #[error("Failed to copy to container {id}: {message}")]
    Copy { id: String, message: String },

    #[error("Unsupported operation '{operation}' for runtime '{runtime}'")]
    UnsupportedOperation { operation: String, runtime: String },

    #[error("CLI command failed: {command} (exit {exit_code}): {stderr}")]
    CliError {
        command: String,
        stderr: String,
        exit_code: i32,
    },

    #[error("Runtime detection failed: {0}")]
    Detection(String),

    #[error("Container not found: {0}")]
    ContainerNotFound(String),

    #[error("Volume not found: {0}")]
    VolumeNotFound(String),

    #[error("Network not found: {0}")]
    NetworkNotFound(String),
}

pub type SharedRuntime = Arc<dyn Runtime>;
pub type SharedContainerRuntime = SharedRuntime;

#[async_trait]
pub trait Runtime: Send + Sync + 'static {
    fn name(&self) -> &str;

    fn capabilities(&self) -> RuntimeCapabilities;

    async fn pull_image(
        &self,
        image: &str,
        tag: &str,
        auth: Option<&ImageAuth>,
    ) -> Result<String, RuntimeError>;

    async fn image_exists(&self, image: &str, tag: &str) -> Result<bool, RuntimeError>;

    async fn create_container(&self, opts: &CreateContainerOpts) -> Result<String, RuntimeError>;
    async fn start_container(&self, id: &str) -> Result<(), RuntimeError>;
    async fn stop_container(&self, id: &str) -> Result<(), RuntimeError>;
    async fn remove_container(&self, id: &str, force: bool) -> Result<(), RuntimeError>;
    async fn list_containers(&self, all: bool) -> Result<Vec<ContainerInfo>, RuntimeError>;

    async fn pause_container(&self, _id: &str) -> Result<(), RuntimeError> {
        Err(RuntimeError::UnsupportedOperation {
            operation: "pause_container".to_string(),
            runtime: self.name().to_string(),
        })
    }

    async fn resume_container(&self, _id: &str) -> Result<(), RuntimeError> {
        Err(RuntimeError::UnsupportedOperation {
            operation: "resume_container".to_string(),
            runtime: self.name().to_string(),
        })
    }

    async fn create_network(&self, name: &str) -> Result<String, RuntimeError>;
    async fn list_networks(&self) -> Result<Vec<NetworkInfo>, RuntimeError>;

    async fn create_volume(&self, name: &str) -> Result<String, RuntimeError>;
    async fn remove_volume(&self, name: &str) -> Result<(), RuntimeError>;

    async fn exec_container(
        &self,
        id: &str,
        cmd: Vec<String>,
        env: Option<Vec<String>>,
    ) -> Result<String, RuntimeError>;

    async fn copy_to_container(
        &self,
        id: &str,
        data: &[u8],
        path: &str,
    ) -> Result<(), RuntimeError>;

    async fn container_logs(
        &self,
        id: &str,
        opts: &LogOpts,
    ) -> Result<mpsc::Receiver<LogEntry>, RuntimeError>;

    async fn container_stats(&self, id: &str) -> Result<ContainerStats, RuntimeError>;

    async fn health_check(&self) -> Result<(), RuntimeError>;
}

pub use Runtime as ContainerRuntime;
