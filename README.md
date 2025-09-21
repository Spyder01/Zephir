# Zephir CLI

Zephir is a Rust-based CLI tool for packaging, unpacking, and invoking application directories in a sandboxed environment. It supports **native binaries** and **WebAssembly (WASM)** artifacts. Zephir provides sandboxing features like CPU time, memory, and storage limits, as well as graceful shutdown.

---

## Features

* **Init**: Create a default configuration file.
* **Package**: Package a directory into a `.zephir` artifact.
* **Unpack**: Unpack a packaged artifact into a sandbox directory.
* **Invoke**: Run the unpacked artifact inside a sandbox.
* **Run**: Full pipeline: unpack → sandbox → invoke.
* **Sandboxing**: Limit CPU, memory, and storage for safe execution.
* **Graceful shutdown**: Cleans up sandbox directories on Ctrl+C.
* **Logging**: Supports logging to stdout or file with configurable prefix and debug mode.
* **WASM Support**: Run WebAssembly modules with CPU, memory, and storage limits.

---

## Installation

Ensure you have Rust installed (Rust 1.86+ recommended).

```bash
git clone <repository-url>
cd zephir-rs
cargo build --release
```

This will produce a binary in `target/release/zephir-rs`.

---

## Configuration

Zephir uses a YAML configuration file (default: `zephir.yaml`).

### Example Default Configuration

```yaml
name: zephir-function
function:
  app:
    entry: ./zephir-function
  bundle:
    packagePath: function.zephir
    artifactType: NATIVE   # or WASM
  resources:
    memory: 134217728   # 128 MB
    storage: 536870912  # 512 MB
    cpuLimit: 10        # 10 seconds
storage:
  sandbox: zephir-sandbox/
  cache: zephir-cache/
logConfig:
  toFile: false
  filePath: null
  toStdout: true
  prefix: "[Zephir]"
  debugEnabled: false
```

> For WASM artifacts, set `artifactType: WASM` and ensure the `.wasm` file exists at `packagePath`.

---

## CLI Usage

### 1. Initialize a configuration

```bash
zephir-rs init --output ./zephir.yaml
```

Creates a default `zephir.yaml` if it doesn’t exist.

---

### 2. Package a directory

```bash
zephir-rs package --dir ./my-function --output ./function.zephir
```

Packages a directory into a `.zephir` artifact.

---

### 3. Unpack an artifact

```bash
zephir-rs unpack --config ./zephir.yaml
```

Unpacks the artifact defined in the configuration to the sandbox directory.

---

### 4. Invoke a sandboxed artifact

```bash
zephir-rs invoke --sandbox ./zephir-sandbox --config ./zephir.yaml --args arg1 arg2
```

* `--sandbox`: Path to sandbox directory.
* `--config`: Path to Zephir configuration.
* `--args`: Arguments to pass to the executable.

> Graceful shutdown is supported via Ctrl+C. The sandbox directory is cleaned automatically.

---

### 5. Run the full pipeline

```bash
zephir-rs run --config ./zephir.yaml
```

* Performs unpack → sandbox → invoke in sequence.
* Cleans up sandbox automatically.
* Logs execution duration and errors.

---

## Logging

* Logs can be printed to **stdout** or written to a **file**.
* Configurable via `logConfig` in the YAML file.

---

## Sandboxing

* CPU time, memory, and disk usage are enforced per `function.resources`.
* Sandbox directory is separate from source and cache directories.
* Automatically cleaned on successful execution or Ctrl+C.

---

## WASM Support

* Execute `.wasm` artifacts directly using the built-in WASM runtime.
* CPU, memory, and storage limits are enforced inside the WASM sandbox.
* Supports standard WASI calls and file system preopening.

Example configuration for WASM:

```yaml
function:
  bundle:
    packagePath: function.wasm
    artifactType: WASM
```

---

## Development

* Clone the repository and build with Cargo:

```bash
cargo build
cargo run -- <COMMAND>
```

* Run tests (if implemented):

```bash
cargo test
```

* Code structure:

```
src/
├─ main.rs          # CLI entrypoint
├─ engine/          # Execution & packaging engines
├─ models/          # Config definitions
├─ utils/           # FS, YAML helpers
├─ logger/          # Logging setup
└─ compress/        # Zstd compression/decompression
```

---

## Requirements

* Rust 1.86+
* Linux / macOS (sandboxing relies on OS-level features)

---

## Future Plans

* **Orchestration and Scaling Layer**: Add support for deploying Zephir functions in a cloud-agnostic serverless style, including bare-metal environments. This would allow multiple concurrent invocations with isolation and scaling.
* **Hermyx Integration**: Integration with [Hermyx](https://github.com/Spyder01/Hermyx) to provide ultra-fast, single-process caching and reverse proxy support for function artifacts.
* **Extended WASM Features**: Improve WASM runtime support including asynchronous calls, streaming input/output, and module caching.

---
