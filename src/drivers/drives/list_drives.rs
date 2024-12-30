use std::fs;
use std::path::PathBuf;

// crate imports
use crate::drivers::drives::{Drive, Drives};

fn get_available_space(path: &PathBuf) -> u64 {
    #[cfg(target_os = "windows")]
    {
        // On Windows, use GetDiskFreeSpaceEx
        use std::os::windows::fs::MetadataExt;
        if let Ok(metadata) = fs::metadata(path) {
            metadata.file_size()
        } else {
            0
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // On Unix systems, use statvfs
        use std::os::unix::fs::MetadataExt;
        if let Ok(stats) = nix::sys::statvfs::statvfs(path) {
            stats.blocks_free() * stats.block_size() as u64
        } else {
            0
        }
    }
}

impl Drive {
    pub fn new(
        name: String,
        path: PathBuf,
        free_space_gb: u64,
        total_space_gb: u64,
        file_system: String,
        os: String,
        is_removable: bool,
        drive_letter: Option<char>,
    ) -> Self {
        Drive {
            name,
            path,
            free_space_gb,
            total_space_gb,
            file_system,
            os,
            is_removable,
            drive_letter,
        }
    }
}

impl Drives {
    pub fn new() -> Self {
        Drives {
            drives: Vec::new()
        }
    }

    pub fn list() -> Self {
        let drives = list_system_drives();
        Drives { drives }
    }

    pub fn get(drive_letter: &str) -> Option<Drive> {
        let drives = Self::list();
        if let Some(letter) = drive_letter.chars().next() {
            drives.drives.into_iter().find(|drive| {
                drive.drive_letter == Some(letter.to_ascii_uppercase())
            })
        } else {
            None
        }
    }
}

fn create_drive(
    path: PathBuf,
    name: String,
    os_type: &str,
    fs_type: &str,
    drive_letter: Option<char>,
) -> Option<Drive> {
    if let Ok(metadata) = fs::metadata(&path) {
        let total_space: u64 = metadata.len();
        let free_space: u64 = get_available_space(&path);
        Some(Drive::new(
            name,
            path,
            free_space / 1_073_741_824, // Convert bytes to GB
            total_space / 1_073_741_824,
            fs_type.to_string(),
            os_type.to_string(),
            false, // Would need system-specific APIs to detect accurately
            drive_letter,
        ))
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
pub fn list_system_drives() -> Vec<Drive> {
    let mut drives = Vec::new();
    for letter in b'A'..=b'Z' {
        let drive_letter: char = letter as char;
        let drive_path: String = format!("{}:\\", drive_letter);
        let path = PathBuf::from(&drive_path);
        if path.exists() {
            if let Some(drive) = create_drive(
                path,
                format!("Drive ({}:)", drive_letter),
                "Windows",
                "NTFS",
                Some(drive_letter),
            ) {
                drives.push(drive);
            }
        }
    }
    drives
}

#[cfg(target_os = "linux")]
pub fn list_system_drives() -> Vec<Drive> {
    let mut drives = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/dev/disk/by-label") {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if let Some(drive) = create_drive(
                    path,
                    entry.file_name().to_string_lossy().to_string(),
                    "Linux",
                    "ext4",
                    None,
                ) {
                    drives.push(drive);
                }
            }
        }
    }
    drives
}

#[cfg(target_os = "macos")]
pub fn list_system_drives() -> Vec<Drive> {
    let mut drives = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/Volumes") {
        for entry in entries.flatten() {
            if let Some(drive) = create_drive(
                entry.path(),
                entry.file_name().to_string_lossy().to_string(),
                "macOS",
                "APFS",
                None,
            ) {
                drives.push(drive);
            }
        }
    }
    drives
}
