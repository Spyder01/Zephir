use crate::models::config;
use chrono::Local;
use fern::Dispatch;
use log;

pub fn setup_logger(cfg: &config::LogConfig) -> Result<(), fern::InitError> {
    let prefix = cfg.prefix.clone().unwrap_or_default();
    let debug_enabled = cfg.debugEnabled;
    let to_stdout = cfg.toStdout;
    let to_file = cfg.toFile;
    let file_path = cfg.filePath.clone();

    let mut base_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                prefix,
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(if debug_enabled {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        });

    if to_stdout {
        base_config = base_config.chain(std::io::stdout());
    }

    if to_file {
        if let Some(path) = file_path {
            base_config = base_config.chain(fern::log_file(path)?);
        }
    }

    base_config.apply()?;
    Ok(())
}

