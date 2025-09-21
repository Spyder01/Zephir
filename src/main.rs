mod utils;
mod models;
mod engine;
mod logger;
mod compress;

use clap::{Parser, Subcommand};
use log::{info, error};
use std::time::Instant;
use std::fs;
use std::path::Path;
use serde_yaml;
use models::config;
use engine::{exec_engine, pack_engine};
use utils::fs::yaml;
use logger::zephir_logger;
use tokio::signal;
use std::sync::Arc;
use tokio::sync::Notify;

/// Zephir CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// initialize the application directory that needs to be packaged.
    Init {
        #[arg(short, long, default_value = "./zephir.yaml")]
        output: String,
    },
    
    /// Package the directory.
    Package {
        #[arg(short, long, default_value = "./test-files")]
        dir: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Unpack the packaged directory.
    Unpack {
        #[arg(short, long, default_value = "./zephir.yaml")]
        config: String,
    },
    
    /// Invoke the unpacked directory
    Invoke {
        #[arg(short, long)]
        args: Vec<String>,
        #[arg(short, long, default_value = "./zephir-sandbox")]
        sandbox: String,
        #[arg(short, long, default_value = "./zephir.yaml")]
        config: String,
    },

    /// Run the full pipeline (unpack + sandbox + invoke)
    Run {
        #[arg(short, long, default_value = "./zephir.yaml")]
        config: String,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let shutdown_notify = Arc::new(Notify::new());
    let shutdown_listener = shutdown_notify.clone();

    // Spawn Ctrl+C listener
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        shutdown_listener.notify_waiters();
    });

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { output } => {
            if Path::new(output).exists() {
                error!("Config file '{}' already exists!", output);
                return;
            }
            let default_config = config::ZephirConfig::sane_defaults();
            match fs::write(output, serde_yaml::to_string(&default_config).unwrap()) {
                Ok(_) => info!("Default config written to {}", output),
                Err(e) => error!("Failed to write default config: {}", e),
            }
        }

        Commands::Package { dir, output } => {
            let package_engine = pack_engine::PackageEngine::new(&dir, output.as_deref());
            match package_engine.package().await {
                Ok(_) => info!("Package successful"),
                Err(e) => error!("Package failed: {}", e),
            }
        }

        Commands::Unpack { config: cfg_path } => {
            let zephir_config = match yaml::parse_yaml_from_file::<config::ZephirConfig>(&cfg_path).await {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to parse config: {}", e);
                    return;
                }
            };
            let engine = exec_engine::ZephirEngine::new(zephir_config);

            match engine.unpack().await {
                Ok(path) => info!("Artifact unpacked to {}", path),
                Err(e) => error!("Unpack failed: {}", e),
            }
        }

        Commands::Invoke { args, sandbox, config: cfg_path } => {
            let zephir_config = match yaml::parse_yaml_from_file::<config::ZephirConfig>(&cfg_path).await {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to parse config: {}", e);
                    return;
                }
            };
            let engine = exec_engine::ZephirEngine::new(zephir_config);

            // Run the actual work as a future
            let engine_clone = Arc::new(engine);
            let sandbox_clone = sandbox.clone();
            let args_clone: Vec<String> = args.clone();

            tokio::select! {
                _ = async {
                    // Synchronous calls wrapped in async
                    if let Err(e) = engine_clone.sandbox(&sandbox_clone) {
                        error!("Failed to setup sandbox: {}", e);
                        return;
                    }

                    if let Err(e) = engine_clone.invoke(
                        &args_clone.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                        &sandbox_clone
                    ).await {
                        error!("Invocation failed: {}", e);
                    }

                    if let Err(e) = engine_clone.cleanup_sandbox(&sandbox_clone) {
                        error!("Cleanup failed: {}", e);
                    }
                } => {},

                _ = shutdown_notify.notified() => {
                    info!("Graceful shutdown requested. Cleaning up...");
                    let _ = engine_clone.cleanup_sandbox(&sandbox_clone);
                }
            }
        }

        Commands::Run { config: cfg_path } => {
            let zephir_config = match yaml::parse_yaml_from_file::<config::ZephirConfig>(&cfg_path).await {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to parse config: {}", e);
                    return;
                }
            };
            let engine = Arc::new(exec_engine::ZephirEngine::new(zephir_config));

            if let Some(log_cfg) = &engine.config.logConfig {
                zephir_logger::setup_logger(log_cfg).unwrap_or_else(|err| {
                    eprintln!("Logger setup failed: {}", err);
                });
                info!("Logger initialized");
            }

            let start = Instant::now();
            info!("Starting full execution pipeline...");

            let engine_clone = engine.clone();

            tokio::select! {
                _ = async {
                    let sandbox_path = match engine_clone.unpack().await {
                        Ok(path) => path,
                        Err(e) => {
                            error!("Failed to unpack sandbox: {}", e);
                            return;
                        }
                    };

                    if let Err(e) = engine_clone.sandbox(&sandbox_path) {
                        error!("Sandbox setup failed: {}", e);
                        return;
                    }

                    if let Err(e) = engine_clone.invoke(&[], &sandbox_path).await {
                        error!("Invocation failed: {}", e);
                    }

                    if let Err(e) = engine_clone.cleanup_sandbox(&sandbox_path) {
                        error!("Cleanup failed: {}", e);
                    }

                    let duration = start.elapsed();
                    info!("Full pipeline completed in {:.2?}", duration);
                } => {},

                _ = shutdown_notify.notified() => {
                    info!("Graceful shutdown requested during Run.");
                    // Note: You could track sandbox path separately if needed
                }
            }
        }
    }
}
