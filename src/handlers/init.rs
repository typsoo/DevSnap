use anyhow::{Context, Result};
use std::fs;

pub fn handle_init() -> Result<()> {
    handle_install_plugin()
}

fn handle_install_plugin() -> Result<()> {
    let home = dirs::home_dir().context("Failed to get home dir")?;
    let ext_dir = home.join(".vscode/extensions/devsnap-companion");

    fs::create_dir_all(&ext_dir)?;

    let package_json = include_str!("../plugin_assets/package.json");
    let extension_js = include_str!("../plugin_assets/extension.js");

    fs::write(ext_dir.join("package.json"), package_json)?;
    fs::write(ext_dir.join("extension.js"), extension_js)?;

    println!("VS Code companion plugin installed successfully!");
    println!("Please restart VS Code for the changes to take effect.");

    Ok(())
}
