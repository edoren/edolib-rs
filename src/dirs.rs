use dirs;
use std::path::{Path, PathBuf};

pub fn config_dir(app_name: String) -> Option<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let dir = if Path::new("/.dockerenv").exists() {
        Some(PathBuf::from("/data"))
    } else {
        dirs::config_dir()
    };

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    let dir = dirs::config_dir();

    dir.map(|p| p.join(app_name))
}

pub fn cache_dir(app_name: String) -> Option<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let dir = if Path::new("/.dockerenv").exists() {
        Some(PathBuf::from("/cache"))
    } else {
        dirs::cache_dir()
    };

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    let dir = dirs::cache_dir();

    dir.map(|p| p.join(app_name))
}
