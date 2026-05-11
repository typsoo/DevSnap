use std::path::PathBuf;
use sysinfo::System;

pub fn explore_processes() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\n--- Scanning for Firefox processes ---");
    let mut firefox_found = false;

    // Iterate through all running processes
    for (pid, process) in sys.processes() {
        let name = process.name().to_ascii_lowercase();

        // Match process name
        if name.contains("firefox") {
            firefox_found = true;
            // Print the Process ID and the exact command used to launch it
            println!("Found Firefox (PID: {}): {:?}", pid, process.cmd());
        }
    }

    if firefox_found {
        println!("\n--- Looking for Firefox session files ---");
        find_firefox_session();
    } else {
        println!("\nFirefox is not currently running.");
    }
}

// Helper function to locate the Firefox recovery.jsonlz4 file
fn find_firefox_session() {
    // Firefox stores profiles in ~/.mozilla/firefox/ on Linux
    if let Some(mut mozilla_dir) = dirs::home_dir() {
        mozilla_dir.push(".mozilla");
        mozilla_dir.push("firefox");

        if mozilla_dir.exists() {
            // Read the contents of the firefox directory
            if let Ok(entries) = std::fs::read_dir(&mozilla_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    // Look for profile directories (usually contain 'default' in the name)
                    if path.is_dir() {
                        let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                        if dir_name.contains("default") {
                            // The active session is stored in sessionstore-backups/recovery.jsonlz4
                            let session_file =
                                path.join("sessionstore-backups").join("recovery.jsonlz4");

                            if session_file.exists() {
                                println!("Active session file found!");
                                println!("Path: {}", session_file.display());
                                println!(
                                    "Note: This file is compressed. You cannot read it directly with serde_json."
                                );
                            }
                        }
                    }
                }
            }
        } else {
            println!(
                "Mozilla Firefox directory not found at {}",
                mozilla_dir.display()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let mut sys = System::new_all();

        sys.refresh_all();

        println!("=> system:");
        // RAM and swap information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

        println!("System name:             {:?}", System::name());
        println!("System kernel version:   {:?}", System::kernel_version());
        println!("System OS version:       {:?}", System::os_version());
        println!("System host name:        {:?}", System::host_name());

        for (pid, process) in sys.processes() {
            println!("[{pid}] {:?}", process.name().to_ascii_lowercase());
        }
    }
}
