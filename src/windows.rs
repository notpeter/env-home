use std::env;
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
    match env::var("USERPROFILE") {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => None,
    }
}

pub fn config_dir() -> Option<PathBuf> {
    match env::var("XDG_CONFIG_HOME") {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => match env::var("APPDATA") {
            Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
            _ => None,
        },
    }
}

pub fn cache_dir() -> Option<PathBuf> {
    match env::var("XDG_CACHE_HOME") {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => match env::var("LOCALAPPDATA") {
            Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
            _ => None,
        },
    }
}
