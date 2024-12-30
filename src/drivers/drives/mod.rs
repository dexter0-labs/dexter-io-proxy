pub mod list_drives;
pub mod get_storage;

use serde::{Serialize, Deserialize};

use std::path::PathBuf;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Drive {
    pub name: String,
    pub path: PathBuf,
    pub free_space_gb: u64,
    pub total_space_gb: u64,
    pub file_system: String,
    pub os: String,
    pub is_removable: bool,
    pub drive_letter: Option<char>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Drives {
    pub drives: Vec<Drive>,
}