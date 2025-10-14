# 🧭 Zephir CLI

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/spyder01/zephir-rs.svg)](https://github.com/spyder01/zephir-rs/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/spyder01/zephir-rs.svg)](https://github.com/spyder01/zephir-rs/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)
[![Hacktoberfest](https://img.shields.io/badge/Hacktoberfest-2025-blueviolet.svg)](https://hacktoberfest.com/)

---

## ⚙️ Overview

**Zephir** is a **Rust-based CLI tool** for packaging, unpacking, and invoking application directories inside a **sandboxed environment**.
It supports **Native binaries**, **WebAssembly (WASM)**, and **Lua scripts**, offering fine-grained resource control and secure isolated execution.

Ideal for **serverless runtimes**, **sandboxed compute**, or **local function testing**.

---

## ✨ Features

* 🏗️ **Init** — Generate a default configuration file.
* 📦 **Package** — Package a directory into a `.zephir` artifact.
* 📂 **Unpack** — Unpack packaged artifacts to a sandbox directory.
* ⚙️ **Invoke** — Run unpacked artifacts inside an isolated sandbox.
* 🚀 **Run** — Full pipeline: *unpack → sandbox → invoke*.
* 🔒 **Sandboxing** — CPU, memory, and storage limits for safe execution.
* 🧹 **Graceful Shutdown** — Cleans up sandbox directories automatically.
* 🪵 **Logging** — Structured logs with prefix and debug support.
* 🌐 **WASM Support** — Run WebAssembly modules using a WASI-compliant runtime.
* 🌀 **Lua Support** — Execute sandboxed Lua scripts securely.

---

## ⚙️ Installation

Ensure **Rust (v1.86+)** is installed.

```bash
git clone https://github.com/spyder01/zephir-rs.git
cd zephir-rs
cargo build --release
```

The compiled binary will be available at:

```
target/release/zephir-rs
```

---

## 🧩 Configuration

Zephir uses a **YAML configuration file** (default: `zephir.yaml`).

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
  toStdout: true
  prefix: "[Zephir]"
  debugEnabled: false
```

Supported `artifactType` values:

* `NATIVE` — Compiled executables
* `WASM` — WebAssembly modules
* `LUA` — Lua scripts

---

## 🧭 CLI Commands

### 🔧 Initialize configuration

```bash
zephir-rs init --output ./zephir.yaml
```

### 📦 Package a directory

```bash
zephir-rs package --dir ./my-function --output ./function.zephir
```

### 📂 Unpack an artifact

```bash
zephir-rs unpack --config ./zephir.yaml
```

### ⚙️ Invoke an artifact

```bash
zephir-rs invoke --sandbox ./zephir-sandbox --config ./zephir.yaml --args arg1 arg2
```

### 🚀 Run (Full Pipeline)

```bash
zephir-rs run --config ./zephir.yaml
```

---

## 🪵 Logging Configuration

| Option         | Description                   |
| -------------- | ----------------------------- |
| `toFile`       | Write logs to a file          |
| `filePath`     | Path to log file (if enabled) |
| `toStdout`     | Print logs to stdout          |
| `prefix`       | Log prefix label              |
| `debugEnabled` | Enables verbose logging       |

---

## 🧱 Sandboxing

Zephir isolates execution using strict sandboxing controls:

* **CPU limit:** via fuel counters or process control
* **Memory cap:** on WASM, Lua, and native executables
* **Storage quota:** per sandbox directory
* **Automatic cleanup:** on completion or interruption

---

## 🧬 Execution Modes

### 🔹 Native Execution

```yaml
artifactType: NATIVE
entry: ./my_binary
```

Runs local executables with real-time stdout/stderr streaming and enforced resource limits.

---

### 🔹 WebAssembly Execution

```yaml
artifactType: WASM
entry: ./module.wasm
```

* Uses **Wasmtime + WASI**
* Preopens `/sandbox` directory
* Enforces CPU, memory, and file I/O limits
* Supports graceful shutdowns

---

### 🔹 Lua Execution

```yaml
artifactType: LUA
entry: ./main.lua
```

Runs sandboxed Lua scripts using [`mlua`](https://crates.io/crates/mlua) with restricted standard libraries.

#### Example

```lua
print("Hello from Lua!")
print("Sandbox path:", sandbox_path)

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

* 🦀 Rust 1.86+
* 🧠 Linux / macOS (Unix sandboxing features)
* 🧩 (Optional) Wasmtime for WASM runtime

---

## 🎯 Roadmap

* [ ] Add network namespace sandboxing
* [ ] WASM async I/O and streaming support
* [ ] Lua execution timeout controls
* [ ] Hermyx integration for cached artifact serving
* [ ] Add example templates for NATIVE / WASM / LUA projects

---

## 🎃 Hacktoberfest 2025 — Contribute & Learn!

We’re participating in **Hacktoberfest 2025**!
If you’re a systems, Rust, or WASM enthusiast — this is the perfect time to contribute 🚀

### 🧩 Good First Issues

Check them out here 👉 [Good First Issues](https://github.com/spyder01/zephir-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

### 🛠 Ways to Contribute

* 🦀 Implement new **sandboxing features**
* ⚙️ Improve **WASM or Lua execution engines**
* 🧪 Add **unit/integration tests**
* 🧾 Improve **documentation and examples**
* 🧰 Build **utility commands** (e.g., resource inspector)
* 🌐 Add **demo projects** for all artifact types

### 🧠 How to Get Started

1. **Fork** the repo
2. **Clone** your fork

   ```bash
   git clone https://github.com/<your-username>/zephir-rs.git
   ```
3. **Create a branch**

   ```bash
   git checkout -b feature/add-wasm-limits
   ```
4. **Build & test**

   ```bash
   cargo run -- init
   cargo test
   ```
5. **Commit & push**

   ```bash
   git commit -m "Add CPU limit enforcement for WASM"
   git push origin feature/add-wasm-limits
   ```
6. **Open a Pull Request** 🎉

---

## 🧑‍💻 Maintainer

**[@spyder01](https://github.com/spyder01)**

---

## 📄 License

Licensed under the **MIT License** — see [LICENSE](./LICENSE) for details.

---

## 🌟 Support the Project

If you like **Zephir**, give it a ⭐ on GitHub!
It helps others discover the project and supports ongoing development.
