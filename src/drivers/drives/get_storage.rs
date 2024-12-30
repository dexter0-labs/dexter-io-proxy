use std::fs;

use std::path::PathBuf;
use crate::drivers::drives::Drive;
use tracing::{info, warn};

impl Drive {
    pub fn get_total_storage(&self) -> u64 {
        info!("Getting total storage for path: {:?}", self.path);
        
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
            use windows::Win32::System::SystemServices::LARGE_INTEGER;
            use std::os::windows::prelude::*;

            let path_str = self.path.to_string_lossy();
            let mut total_bytes = LARGE_INTEGER::default();
            let mut free_bytes = LARGE_INTEGER::default();
            let mut available_bytes = LARGE_INTEGER::default();

            unsafe {
                if GetDiskFreeSpaceExW(
                    &path_str,
                    &mut available_bytes,
                    &mut total_bytes,
                    &mut free_bytes,
                ).as_bool() {
                    let size = total_bytes.QuadPart() / (1024 * 1024 * 1024); // Convert to GB
                    info!("Windows total storage: {}GB", size);
                    size as u64
                } else {
                    warn!("Failed to get Windows disk space for path: {:?}", self.path);
                    0
                }
            }
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            if let Ok(stats) = nix::sys::statvfs::statvfs(&self.path) {
                let size = (stats.blocks() * stats.block_size() as u64) / (1024 * 1024 * 1024);
                info!("Unix total storage: {}GB", size);
                size
            } else {
                warn!("Failed to get Unix stats for path: {:?}", self.path);
                0
            }
        }
    }

    pub fn get_free_storage(&self) -> u64 {
        info!("Getting free storage for path: {:?}", self.path);

        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
            use windows::Win32::System::SystemServices::LARGE_INTEGER;
            use std::os::windows::prelude::*;

            let path_str = self.path.to_string_lossy();
            let mut total_bytes = LARGE_INTEGER::default();
            let mut free_bytes = LARGE_INTEGER::default();
            let mut available_bytes = LARGE_INTEGER::default();

            unsafe {
                if GetDiskFreeSpaceExW(
                    &path_str,
                    &mut available_bytes,
                    &mut total_bytes,
                    &mut free_bytes,
                ).as_bool() {
                    let free = free_bytes.QuadPart() / (1024 * 1024 * 1024); // Convert to GB
                    info!("Windows free storage: {}GB", free);
                    free as u64
                } else {
                    warn!("Failed to get Windows disk space for path: {:?}", self.path);
                    0
                }
            }
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            if let Ok(stats) = nix::sys::statvfs::statvfs(&self.path) {
                let free = (stats.blocks_free() * stats.block_size() as u64) / (1024 * 1024 * 1024);
                info!("Unix free storage: {}GB", free);
                free
            } else {
                warn!("Failed to get Unix stats for path: {:?}", self.path);
                0
            }
        }
    }
}
