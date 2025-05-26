use base64::{Engine as _, engine::general_purpose};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn get_jetbrains_config_dir() -> Option<PathBuf> {
    // Try standard platform directories first
    let standard_path = dirs::config_dir()?.join("JetBrains");

    if standard_path.exists() {
        return Some(standard_path);
    }

    // // Try fallback paths
    // let fallback_paths = match () {
    //     _ if cfg!(windows) => &[
    //         r"C:\Users\%USERNAME%\AppData\Roaming\JetBrains",
    //         r"C:\Users\%USERNAME%\.jetbrains",
    //     ],
    //     _ if cfg!(target_os = "macos") => &[
    //         "~/Library/Application Support/JetBrains",
    //         "~/.jetbrains",
    //     ],
    //     _ => &[
    //         "~/.config/JetBrains",
    //         "~/.jetbrains",
    //     ],
    // };
    //
    // fallback_paths
    //     .iter()
    //     .filter_map(|&path_str| {
    //         let expanded = if cfg!(windows) {
    //             let username = std::env::var("USERNAME").ok()?;
    //             path_str.replace("%USERNAME%", &username)
    //         } else {
    //             path_str.to_string()
    //         };
    //
    //         shellexpand::full(&expanded)
    //             .ok()
    //             .map(|cow| PathBuf::from(cow.as_ref()))
    //     })
    //     .find(|path| path.exists())
    None
}

fn update_id_file(file_path: &Path) -> Result<()> {
    println!("Updating file: {}", file_path.display());

    let new_uuid = Uuid::new_v4().to_string();
    println!("New UUID: {}", new_uuid);

    // Show old UUID if it exists
    if file_path.exists() {
        let old_uuid = fs::read_to_string(file_path).unwrap_or_default();
        if !old_uuid.is_empty() {
            println!("Old UUID: {}", old_uuid);
        }
    }

    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // Delete the file if it exists
    if file_path.exists() {
        let _ = fs::remove_file(file_path);
    }

    // Write new UUID to file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(new_uuid.as_bytes())?;
    println!("Successfully wrote new UUID to file");
    Ok(())
}

fn run() -> Result<()> {
    let jetbrains_dir = get_jetbrains_config_dir()
        .ok_or("Failed to find JetBrains configuration directory")?;

    println!("JetBrains config directory: {}", jetbrains_dir.display());

    let id_files = ["UGVybWFuZW50RGV2aWNlSWQ=", "UGVybWFuZW50VXNlcklk"];

    for file_name in &id_files {
        let file_path = jetbrains_dir.join(String::from_utf8(general_purpose::STANDARD.decode(file_name)?)?);
        update_id_file(&file_path)?;
        lock_file(&file_path)?;
    }

    println!("ID files have been updated and locked successfully!");
    Ok(())
}

fn lock_file(file_path: &Path) -> Result<()> {
    println!("Locking file: {}", file_path.display());

    if !file_path.exists() {
        return Err(format!("File doesn't exist, can't lock: {}", file_path.display()).into());
    }

    // Use platform-specific commands to lock the file
    let _ = if cfg!(windows) {
        Command::new("attrib")
            .args(["+R", &file_path.to_string_lossy()])
            .output()
    } else {
        Command::new("chmod")
            .args(["444", &file_path.to_string_lossy()])
            .output()
    };

    // Always ensure file is read-only using Rust API regardless of platform command result
    let mut perms = fs::metadata(file_path)?.permissions();
    perms.set_readonly(true);
    fs::set_permissions(file_path, perms)?;

    println!("Successfully locked file");
    Ok(())
}