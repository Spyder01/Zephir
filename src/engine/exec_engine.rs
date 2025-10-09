use tokio::{
    io::{AsyncBufReadExt, BufReader},
    join,
    process,
};
use std::{fs, io, path::Path, process::Stdio};
use log::{info, error};
use wasmtime::*;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};
use mlua::{Lua, StdLib, LuaOptions};
use thiserror::Error;

use crate::models::config;
use crate::utils::fs::{fs_crud, path};
use crate::compress::compress_zstd;
use crate::utils::os::{os_info, os_sandbox};

#[derive(Error, Debug)]
pub enum ZephirInvokationError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("WASM execution error: {0}")]
    Wasm(#[from] wasmtime::Error),

    #[error("Lua execution error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug)]
pub struct ZephirEngine {
    pub config: config::ZephirConfig,
}

impl ZephirEngine {
    pub fn new(config: config::ZephirConfig) -> Self {
        Self { config }
    }

    /// Unpack the artifact into the sandbox directory.
    pub async fn unpack(&self, no_cache: bool) -> io::Result<String> {
        let sane_storage_defaults = config::StorageConfig::sane_defaults();
        let storage_config = self.config.storage.as_ref().unwrap_or(&sane_storage_defaults);

        info!(
            "Ensuring cache directory: {} and storage-directory: {} exists.",
            storage_config.cache.as_deref().unwrap_or(sane_storage_defaults.cache.as_deref().unwrap()),
            storage_config.sandbox.as_deref().unwrap_or(sane_storage_defaults.sandbox.as_deref().unwrap())
        );

        let (res1, res2) = join!(
            fs_crud::ensure_dir(storage_config.cache.as_deref().unwrap_or(sane_storage_defaults.cache.as_deref().unwrap())),
            fs_crud::ensure_dir(storage_config.sandbox.as_deref().unwrap_or(sane_storage_defaults.sandbox.as_deref().unwrap()))
        );
        res1?;
        res2?;

        let cache_path = Path::new(storage_config.cache.as_deref().unwrap_or(sane_storage_defaults.cache.as_deref().unwrap()));
        let artifact_cache_path = path::get_artifact_cache(&cache_path);

        let sandbox_dir_path = Path::new(storage_config.sandbox.as_deref().unwrap_or(sane_storage_defaults.sandbox.as_deref().unwrap()));
        let sandbox_path = path::get_atomic_sandbox_path(&sandbox_dir_path);

        if !no_cache && !fs_crud::dir_exists(&artifact_cache_path).await {
            compress_zstd::decompress_zstd_to_dir(
                &self.config.function.bundle.packagePath,
                artifact_cache_path.to_str().expect("Invalid file path"),
            )?;
        }

        if no_cache {
            compress_zstd::decompress_zstd_to_dir(&self.config.function.bundle.packagePath, sandbox_path.to_str().expect("Invalid file path"))?;
        } else {
            fs_crud::copy_dir_recursive(&artifact_cache_path, &sandbox_path)?;
        }

        Ok(sandbox_path.to_str().unwrap().to_string())
    }

    /// Apply sandbox restrictions: CPU time, memory, and file size.
    pub fn sandbox(&self, sandbox_path_str: &str) -> io::Result<()> {
        let sandbox_path = Path::new(sandbox_path_str);

        os_sandbox::apply_unix_sandbox(
            os_info::has_root_privilege(),
            Some(&sandbox_path),
            self.config.function.resources.cpuLimit,   // CPU time limit (seconds)
            self.config.function.resources.memory,     // max address space
            self.config.function.resources.storage     // max file size
        )?;

        Ok(())
    }

    /// Invoke a binary or script inside the sandbox and stream stdout/stderr.
    pub async fn invoke(&self, args: &[&str], sandbox_path: &str) -> Result<(), ZephirInvokationError> {
        match self.config.function.bundle.artifactType {
            config::ArtifactType::NATIVE => self.invoke_native(args, sandbox_path).await,
            config::ArtifactType::WASM => self.invoke_wasm(sandbox_path).await,
            config::ArtifactType::LUA => self.invoke_lua(sandbox_path).await,
        }
    }

    /// Invoke a native binary.
    pub async fn invoke_native(&self, args: &[&str], sandbox_path: &str) -> Result<(), ZephirInvokationError> {
        let sandbox_dir = Path::new(sandbox_path);

        let mut child = process::Command::new(&self.config.function.app.entry)
            .args(args)
            .current_dir(sandbox_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Child did not have stdout");
        let stderr = child.stderr.take().expect("Child did not have stderr");

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        loop {
            tokio::select! {
                line = stdout_reader.next_line() => match line? {
                    Some(l) => info!("[{}_info] {}", self.config.name, l),
                    None => break,
                },
                line = stderr_reader.next_line() => match line? {
                    Some(l) => error!("[{}_error] {}", self.config.name, l),
                    None => break,
                },
            }
        }

        let status = child.wait().await?;
        if !status.success() {
            return Err(ZephirInvokationError::Other(format!("Native process exited with {}", status)));
        }

        Ok(())
    }

    /// Invoke a WASM module using wasmtime + WASI.
    pub async fn invoke_wasm(&self, sandbox_path: &str) -> Result<(), ZephirInvokationError> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, &self.config.function.app.entry)?;

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(
                Path::new(sandbox_path),
                "/sandbox",
                DirPerms::all(),
                FilePerms::all(),
            )?
            .build();

        let mut store = Store::new(&engine, wasi);

        if self.config.function.resources.cpuLimit > 0 {
            store.set_fuel(self.config.function.resources.cpuLimit * 1_000_000)?;
        }

        let instance = Instance::new(&mut store, &module, &[])?;
        let start_func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;

        info!("[{}] Starting WASM module", self.config.name);
        start_func.call(&mut store, ())?;
        info!("[{}] WASM module finished", self.config.name);

        Ok(())
    }

    /// Invoke a Lua script.
    pub async fn invoke_lua(&self, sandbox_path: &str) -> Result<(), ZephirInvokationError> {
        let script_path = Path::new(sandbox_path).join(&self.config.function.app.entry);
        let script = fs::read_to_string(script_path)?;


        let lua = Lua::new_with(
            StdLib::ALL_SAFE,
            LuaOptions::default(),
        )?;

        let globals = lua.globals();
        globals.set("sandbox_path", sandbox_path)?;
        globals.set(
            "print",
            lua.create_function(|_, msg: String| {
                info!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        info!("[{}] Starting Lua script", self.config.name);

        let chunk = lua.load(&script).set_name("user_script");
        chunk.exec()?;

        info!("[{}] Lua script finished", self.config.name);

        Ok(())
    }

    /// Clean up the sandbox directory after execution.
    pub fn cleanup_sandbox(&self, sandbox_path: &str) -> io::Result<()> {
        let path = Path::new(sandbox_path);
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }
}
