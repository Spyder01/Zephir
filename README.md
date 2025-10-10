# Zephir CLI

Zephir is a **Rust-based CLI tool** for packaging, unpacking, and invoking application directories in a **sandboxed environment**.
It supports **Native binaries**, **WebAssembly (WASM)**, and **Lua scripts**, providing fine-grained resource control and safe isolated execution.

---

## ✨ Features

* **Init** — Generate a default configuration file.
* **Package** — Package a directory into a `.zephir` artifact.
* **Unpack** — Unpack packaged artifacts to a sandbox directory.
* **Invoke** — Run the unpacked artifact inside a sandboxed environment.
* **Run** — Full pipeline: unpack → sandbox → invoke.
* **Sandboxing** — Limit **CPU time**, **memory**, and **storage** for safe isolated execution.
* **Graceful Shutdown** — Automatically cleans up sandbox directories on `Ctrl+C`.
* **Logging** — Structured logs to stdout or file, with configurable prefix and debug mode.
* **WASM Support** — Run WebAssembly modules in a WASI-compliant runtime.
* **Lua Support** — Execute sandboxed Lua scripts with safe standard libraries and Zephir-integrated logging.

---

## ⚙️ Installation

Ensure Rust (v1.86+) is installed:

```bash
git clone <repository-url>
cd zephir-rs
cargo build --release
```

This builds the binary at `target/release/zephir-rs`.

---

## 🧩 Configuration

Zephir uses a YAML configuration file (default: `zephir.yaml`).

### Example Configuration

```yaml
name: zephir-function
function:
  app:
    entry: ./main.lua
  bundle:
    packagePath: function.zephir
    artifactType: LUA   # NATIVE | WASM | LUA
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

> Supported `artifactType`:
>
> * `NATIVE` — for binaries/executables
> * `WASM` — for `.wasm` modules
> * `LUA` — for `.lua` scripts

---

## 🧭 CLI Commands

### 1. Initialize configuration

```bash
zephir-rs init --output ./zephir.yaml
```

Generates a default configuration if it doesn’t exist.

---

### 2. Package a directory

```bash
zephir-rs package --dir ./my-function --output ./function.zephir
```

Packages a directory into a `.zephir` compressed artifact.

---

### 3. Unpack an artifact

```bash
zephir-rs unpack --config ./zephir.yaml
```

Unpacks the artifact defined in the config into the sandbox directory.

---

### 4. Invoke an artifact

```bash
zephir-rs invoke --sandbox ./zephir-sandbox --config ./zephir.yaml --args arg1 arg2
```

Runs the unpacked artifact in the sandbox.
Graceful shutdown is supported — `Ctrl+C` automatically cleans the sandbox.

---

### 5. Run (Full Pipeline)

```bash
zephir-rs run --config ./zephir.yaml
```

Runs the **unpack → sandbox → invoke** pipeline automatically.

---

## 🪵 Logging

Zephir logs can be customized via `logConfig`:

| Option         | Description              |
| -------------- | ------------------------ |
| `toFile`       | Write logs to file       |
| `filePath`     | Path to log file         |
| `toStdout`     | Print logs to stdout     |
| `prefix`       | Log prefix label         |
| `debugEnabled` | Enables debug-level logs |

---

## 🧱 Sandboxing

Zephir enforces:

* **CPU limits** via execution fuel or process control.
* **Memory caps** on WASM/Lua/Native processes.
* **Storage limits** per sandbox directory.

Sandbox directories are **automatically cleaned** after completion or interruption.

---

## 🧬 Execution Modes

### 1. Native Execution

Runs compiled executables (`artifactType: NATIVE`).

```yaml
artifactType: NATIVE
entry: ./my_binary
```

Streams stdout/stderr in real time, respecting resource limits.

---

### 2. WebAssembly Execution

Runs `.wasm` modules using **Wasmtime + WASI**.

```yaml
artifactType: WASM
entry: ./module.wasm
```

* Supports WASI system calls.
* Preopens sandbox directory (`/sandbox`).
* CPU, memory, and file system limits enforced.
* Graceful start/stop with Zephir logging.

---

### 3. Lua Execution

Runs `.lua` scripts safely inside a sandboxed Lua runtime.

```yaml
artifactType: LUA
entry: ./main.lua
```

* Uses [`mlua`](https://crates.io/crates/mlua) with **safe standard libraries only**.
* `print()` is redirected to Zephir’s logger.
* Access to sandboxed paths only.
* `sandbox_path` is exposed to the Lua script’s global scope.

#### Example Lua Script

```lua
print("Hello from Lua!")
print("Sandbox path:", sandbox_path)

-- Example: create a file in sandbox
local f = io.open(sandbox_path .. "/output.txt", "w")
f:write("Lua execution complete.")
f:close()
```

---

## 🧠 Development

```bash
cargo run -- <COMMAND>
cargo test
```

### Project Layout

```
src/
├─ main.rs          # CLI entrypoint
├─ engine/          # Core execution logic
├─ models/          # Config & data structures
├─ utils/           # FS, YAML, OS helpers
├─ logger/          # Logging setup
└─ compress/        # Zstd compression/decompression
```

---

## 🧰 Requirements

* Rust 1.86+
* Linux / macOS (sandbox uses Unix features)
* (Optional) Wasmtime for WASM runtime

---

## 🚀 Future Plans

* **Orchestration Layer**: Cloud/serverless scaling for concurrent invocations.
* **Hermyx Integration**: Use [Hermyx](https://github.com/Spyder01/Hermyx) for ultra-fast caching and proxying of function artifacts.
* **Extended WASM Runtime**: Async I/O, streaming, module caching.
* **Lua Sandboxing Enhancements**: Support user-defined safe APIs, timeouts, and isolated FS contexts.

