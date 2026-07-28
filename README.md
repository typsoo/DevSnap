# DevSnap 📦

DevSnap is a command-line tool written in Rust that allows users to save and restore complete development workspaces on Linux systems using the GNOME desktop environment.

## 🎯 Features

- **Snapshot Creation**: Save running applications, window titles, positions, working directories, and launch commands.
- **Workspace Restoration**: Restore a previously saved workspace layout by launching required applications and opening project directories.
- **Workspace Management**: Group and manage multiple workspace snapshots by project name.
- **Portable Storage**: Snapshots are saved as structured JSON files locally in your user directory.

## ⚙️ Requirements

- **Operating System**: Linux
- **Desktop Environment**: GNOME (primarily)
- **Shell**: Bash/Zsh compatible

## 🚀 Installation

Make sure you have Rust and Cargo installed.

```bash
git clone <repository-url>
cd DevSnap
cargo build --release
```

The executable will be available in `target/release/devsnap`. You can move it to your `~/.local/bin` for global access.

## 💻 CLI Commands & Usage

DevSnap provides a simple, intuitive command-line interface.

### `save`

Captures the current workspace state and saves it as a snapshot.

```bash
devsnap save <workspace-name>
```

_Stores: Running applications, window titles, positions, working directories, and launch commands._

### `restore`

Restores a previously saved workspace.

```bash
devsnap restore <workspace-name>
```

_Action: Launches required applications, opens project directories, and restores window layouts._

### `list`

Displays all currently saved workspace snapshots.

```bash
devsnap list
```

### `show`

Displays detailed information about a specific saved workspace snapshot.

```bash
devsnap show <workspace-name>
```

### `delete`

Removes a saved workspace snapshot.

```bash
devsnap delete <workspace-name>
```

## 🧩 Architecture

The application is structured into several modular components to ensure separation of concerns:

- **CLI Layer**: Parses commands and arguments using `clap`.
- **Snapshot Manager**: Coordinates saving workspace states and loading snapshot files.
- **System Interaction Layer**: Interacts with the Linux OS to list running processes, read window info, and launch applications (leveraging `sysinfo`).
- **Storage Layer**: Handles JSON serialization and file management using `serde` and `serde_json`. Files are saved in `~/local/share/devsnap/snapshots/`.

## 📦 Dependencies

DevSnap is built with the following core Rust libraries:

- [`anyhow`](https://crates.io/crates/anyhow): Flexible error handling.
- [`clap`](https://crates.io/crates/clap): Robust command-line argument parsing.
- [`dirs`](https://crates.io/crates/dirs): Locating standard system directories.
- [`serde`](https://crates.io/crates/serde) & [`serde_json`](https://crates.io/crates/serde_json): Serialization and deserialization of workspace snapshots.
- [`sysinfo`](https://crates.io/crates/sysinfo): Querying system information and running processes.
- [`rust-ini`](https://crates.io/crates/rust-ini): Parsing INI configuration files.
- [`lz4_flex`](https://crates.io/crates/lz4_flex): High-performance data compression.
- [`inquire`](https://crates.io/crates/inquire): Interactive CLI prompts.
