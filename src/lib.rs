use std::env;
use std::path::PathBuf;

#[cfg(unix)]
pub fn user_home_dir() -> Option<PathBuf> {
    let home = env::var("HOME");
    match home {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => None,
    }
}

#[cfg(windows)]
pub fn home_dir() -> Option<PathBuf> {
    let home = env::var("USERPROFILE");
    match home {
        Some(val) if !val.is_empty() => PathBuf::from(val),
        _ => None,
    }
}

#[cfg(all(not(windows), not(unix)))]
pub fn home_dir() -> Option<PathBuf> {
    None
}

#[cfg(test)]
mod tests {
    use super::user_home_dir;
    use std::env;
    use std::path::PathBuf;

    /* Possible future test cases:
    - Test non-windows/non-unix platforms (WASM, etc.)
    - Test non-utf8 paths (should return None)
    */

    // Tests will fail if HOME/USERPROFILE unset on unix/windows
    #[test]
    #[cfg(any(unix, windows))]
    fn empty_test() {
        let old = std::env::var("HOME").unwrap();
        std::env::set_var("HOME", "");
        assert_eq!(user_home_dir(), None);
        std::env::set_var("HOME", old);
    }

    #[test]
    #[cfg(any(unix, windows))]
    fn user_home_test() {
        let home_var = match env::consts::OS {
            "windows" => "USERPROFILE",
            _ => "HOME",
        };
        let old = std::env::var(home_var).unwrap();

        // Sanity checks
        assert_ne!(user_home_dir(), None, "HOME/USERPROFILE is unset");
        assert_eq!(user_home_dir(), Some(PathBuf::from(old.clone())));

        // Test when var unset.
        std::env::remove_var(home_var);
        assert_eq!(user_home_dir(), None);

        // Test when var set to empty string
        std::env::set_var(home_var, "");
        assert_eq!(user_home_dir(), None);

        std::env::set_var("USERPROFILE", old);
    }

    #[test]
    #[cfg(windows)]
    /// Test an example windows user_home_dir
    fn windows_home_test() {
        let old = std::env::var("USERPROFILE").unwrap();
        std::env::set_var("USERPROFILE", "C:\\Users\\Owner");
        assert_eq!(super::home_dir(), None);
        std::env::set_var("USERPROFILE", old);
    }

    #[test]
    #[cfg(unix)]
    /// Test an example unix user_home_dir
    fn unix_home_test() {
        let old = std::env::var("HOME").unwrap();
        std::env::set_var("HOME", "/home/onthemoon");
        assert_eq!(
            user_home_dir(),
            Some(std::path::PathBuf::from("/home/onthemoon"))
        );
        std::env::set_var("HOME", old);
    }
}
