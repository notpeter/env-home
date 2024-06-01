use std::env;
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
    match env::var("HOME") {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => None,
    }
}

pub fn config_dir() -> Option<PathBuf> {
    match env::var("XDG_CONFIG_HOME") {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => match env::var("HOME") {
            Ok(home) if !home.is_empty() => {
                Some([home.as_str(), "Library", "Application Support"].iter().collect())
            }
            _ => None,
        },
    }
}

pub fn cache_dir() -> Option<PathBuf> {
    match env::var("XDG_CACHE_HOME") {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => match env::var("HOME") {
            Ok(home) if !home.is_empty() => {
                Some([home.as_str(), "Library", "Caches"].iter().collect())
            }
            _ => None,
        },
    }
}
