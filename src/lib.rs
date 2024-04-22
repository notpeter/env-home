use std::env;
use std::path::PathBuf;

#[cfg(unix)]
pub fn env_home_dir() -> Option<PathBuf> {
    let home = env::var("HOME");
    match home {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => None,
    }
}

#[cfg(windows)]
pub fn env_home_dir() -> Option<PathBuf> {
    let home = env::var("USERPROFILE");
    match home {
        Ok(val) if !val.is_empty() => Some(PathBuf::from(val)),
        _ => None,
    }
}

#[cfg(all(not(windows), not(unix)))]
pub fn env_home_dir() -> Option<PathBuf> {
    None
}

#[cfg(test)]
mod tests {
    use super::env_home_dir;
    use std::env;
    use std::path::PathBuf;

    /*
    Note! Do not run these tests in parallel, as they modify the environment.
    By default `cargo test` will run tests in parallel (multi-threaded) which
    is unsafe and will cause intermittent panics. To run tests sequentially
    use `cargo test -- --test-threads=1`.

    More info:
    - https://doc.rust-lang.org/std/env/fn.set_var.html
    - https://github.com/rust-lang/rust/issues/27970

    Possible future test cases:
    - Test non-windows/non-unix platforms (WASM, etc.)
    - Test non-utf8 paths (should return None)
    */

    #[test]
    #[cfg(any(unix, windows))]
    fn env_home_test() {
        let home_var = if cfg!(windows) {
            "USERPROFILE"
          } else {
            "HOME"
          };
        let old = std::env::var(home_var).unwrap();

        // Sanity checks
        assert_ne!(env_home_dir(), None, "HOME/USERPROFILE is unset");
        assert_eq!(env_home_dir(), Some(PathBuf::from(old.clone())));

        // Test when var unset.
        env::remove_var(home_var);
        assert_eq!(env_home_dir(), None);

        // Test when var set to empty string
        env::set_var(home_var, "");
        assert_eq!(env_home_dir(), None);

        // Tests a sensible platform specific home directory.
        let temp_dir = if cfg!(windows) { "C:\\temp" } else { "/tmp" };
        std::env::set_var(home_var, temp_dir);
        assert_eq!(env_home_dir(), Some(std::path::PathBuf::from(temp_dir)));

        env::set_var(home_var, old);
    }
}
