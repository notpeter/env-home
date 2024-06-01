// Copyright Peter Tripp

use std::path::PathBuf;

#[cfg(all(unix, not(target_os = "macos")))]
mod unix;

#[cfg(all(unix, not(target_os = "macos")))]
use crate::unix as sys;

#[cfg(all(unix, target_os = "macos"))]
mod mac;

#[cfg(all(unix, target_os = "macos"))]
use crate::mac as sys;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use crate::windows as sys;

#[cfg(all(not(windows), not(unix)))]
mod other;

#[cfg(all(not(windows), not(unix)))]
use crate::other as sys;

/// Returns the path of the current user’s home directory.
/// On Unix, it returns the value of the `HOME` environment variable.
/// On Windows, it returns the value of the `USERPROFILE` environment variable.
pub fn user_home_dir() -> Option<PathBuf> {
    sys::home_dir()
}

/// Returns the path of the current user’s config directory.
/// On Unix use `XDG_CONFIG_HOME` if set, otherwise `$HOME/.config`.
/// On Windows use `XDG_CONFIG_HOME` if set, otherwise `%APPDATA%`
pub fn user_config_dir() -> Option<PathBuf> {
    sys::config_dir()
}

/// Returns the path of the current user’s cache directory.
/// On Unix use `XDG_CACHE_HOME` if set, otherwise `$HOME/.cache`.
/// On Windows use `XDG_CACHE_HOME` if set, otherwise `%LOCALAPPDATA%`
pub fn user_cache_dir() -> Option<PathBuf> {
    sys::cache_dir()
}

#[cfg(test)]
mod tests {
    use super::user_home_dir;
    use std::env;
    use std::path::PathBuf;

    /*
    Note! Do not run these tests in parallel, as they modify the environment.
    By default `cargo test` will run tests in parallel (multi-threaded) which
    is unsafe and will cause intermittent panics. To run tests sequentially
    use `cargo test -- --test-threads=1`.

    More info:
    - https://github.com/rust-lang/rust/issues/124866
    - https://doc.rust-lang.org/std/env/fn.set_var.html
    - https://github.com/rust-lang/rust/issues/27970

    Possible future test cases:
    - Test non-windows/non-unix platforms (WASM, etc.)
    - Test non-utf8 paths (should return None)
    */

    #[cfg(any(unix, windows))]
    #[test]
    fn env_home_test() {
        let home_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        let old = env::var(home_var).unwrap();

        // Sanity checks
        assert_ne!(user_home_dir(), None, "HOME/USERPROFILE is unset");
        assert_eq!(user_home_dir(), Some(PathBuf::from(old.clone())));

        // Test when var unset.
        env::remove_var(home_var);
        assert_eq!(user_home_dir(), None);

        // Test when var set to empty string
        #[allow(unused_unsafe)]
        unsafe {
            env::set_var(home_var, "");
        }
        assert_eq!(user_home_dir(), None);

        // Tests a sensible platform specific home directory.
        let temp_dir = if cfg!(windows) { "C:\\temp" } else { "/tmp" };
        #[allow(unused_unsafe)]
        unsafe {
            env::set_var(home_var, temp_dir);
        }
        assert_eq!(user_home_dir(), Some(PathBuf::from(temp_dir)));

        #[allow(unused_unsafe)]
        unsafe {
            env::set_var(home_var, old);
        }
    }
}
