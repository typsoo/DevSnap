📦 Project Specification
DevSnap — Developer Workspace Snapshot CLI for Linux

1. Project Overview

DevSnap is a command-line tool written in Rust that allows users to save and restore complete development workspaces on Linux systems using the GNOME desktop environment.

The application captures the current workspace state, including open windows, running processes, project directories, and optionally browser sessions, and stores it as a snapshot. These snapshots can later be restored to recreate the same development environment.

This tool is intended to improve developer productivity by enabling quick switching between complex work setups such as different programming projects.

🎯 2. Goals

The main goals of the project are:

Provide a CLI interface for workspace snapshot management
Save and restore running applications and window layouts
Store workspace data in portable configuration files
Allow grouping workspaces by project name
Demonstrate Rust systems programming capabilities
🖥️ 3. Target Environment

The application will run on:

Operating System: Linux
Desktop Environment: primarily GNOME
Shell: Bash/Zsh compatible

⚙️ 4. Functional Requirements
4.1 CLI Commands

The program will expose the following CLI commands.

Save Workspace
devsnap save <workspace-name>

Captures the current workspace state and saves it as a snapshot.

Stored information includes:

running applications
window titles
window positions
working directories
command used to launch application
Restore Workspace
devsnap restore <workspace-name>

Restores a previously saved workspace by:

launching required applications
opening project directories
restoring window positions
List Workspaces
devsnap list

Displays all saved workspace snapshots.

Example output:

## Available Workspaces

rust-course
web-project
machine-learning
Delete Workspace
devsnap delete <workspace-name>

Removes a saved workspace snapshot.

Show Workspace Details
devsnap show <workspace-name>

Displays detailed information about a saved workspace.

💾 5. Data Storage

Workspace snapshots will be stored locally in the user directory:

~/local/share/devsnap/

Structure:

~/local/share/devsnap/
snapshots/
rust-course.json
web-project.json

Each snapshot file contains structured information about the workspace.

Example snapshot file:

{
"workspace_name": "rust-course",
"windows": [
{
"application": "code",
"workspace": 1,
"command": "code ~/projects/rust-course"
},
{
"application": "firefox",
"workspace": 2,
"command": "firefox https://doc.rust-lang.org"
}
]
}

Serialization will be implemented using Serde.

🧩 6. System Architecture

The application will be organized into modular components.

CLI Layer

Responsible for parsing commands using:

clap
Snapshot Manager

Handles:

saving workspace states
loading snapshot files
managing snapshot storage
System Interaction Layer

Responsible for interacting with the OS.

Tasks include:

listing running processes
reading window information
launching applications

Possible libraries:

sysinfo
Storage Layer

Handles serialization and file management.

🧠 7. Optional Advanced Features

These features may be implemented if time allows.

Browser Session Capture

Detect open tabs from Mozilla Firefox by reading session files.

This allows restoring documentation or research tabs used during development.

VSCode Workspace Detection

Detect running instances of Visual Studio Code and save the opened project directory.

Multi-Monitor Layout

Store window positions across monitors and restore them appropriately.

Automation API

Expose a simple HTTP API allowing external tools to control workspace snapshots.

Example endpoint:

POST /workspace/save
POST /workspace/restore

This enables integration with autom
