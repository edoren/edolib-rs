use std::path::PathBuf;

use thiserror::Error;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    EnvFilter, Layer, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::dirs;

#[derive(Error, Debug)]
pub enum LogError {
    #[error("Could not setup logging: {0}")]
    SetupFailed(String),
}

pub async fn setup(app_name: &str) -> Result<(), LogError> {
    setup_with_folder(app_name, None).await
}

pub async fn setup_with_folder(
    app_name: &str,
    config_folder: Option<PathBuf>,
) -> Result<(), LogError> {
    let app_config_dir = if let Some(config_folder) = config_folder {
        config_folder.join(app_name)
    } else {
        dirs::config_dir(&app_name).ok_or(LogError::SetupFailed(
            "Could not find configuration directory".into(),
        ))?
    };

    let logs_dir = app_config_dir.join("logs");
    let default_filter = |filter: LevelFilter| {
        EnvFilter::builder()
            .with_default_directive(filter.into())
            .from_env_lossy()
    };

    let file_appender = RollingFileAppender::builder()
        .max_log_files(7)
        .rotation(Rotation::DAILY)
        .filename_prefix(app_name)
        .filename_suffix("log")
        .build(logs_dir.clone())
        .map_err(|e| LogError::SetupFailed(format!("{e}")))?;
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(default_filter(LevelFilter::DEBUG))
        .boxed();

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_filter(default_filter(LevelFilter::INFO))
        .boxed();

    let mut layers = Vec::new();
    layers.push(file_layer);
    layers.push(stdout_layer);
    tracing_subscriber::registry().with(layers).init();

    Ok(())
}
