# env_home rust crate

A Rust crate providing a platform independent method to identify user's home directory
using environment variables: `HOME` in Unix and `USERPROFILE` in Windows.

## Description

This can be used as drop-in replacement for the deprecated `std::env::home_dir`
from the rust standard library -- if you have HOME or USERPROFILE set appropriately.

Unlike [`std::env::home_dir`](https://doc.rust-lang.org/std/env/fn.home_dir.html)
in the standard library this crate only looks at enviornment variables
and does not fallback to other platform specific APIs when they are unset.

This functionality is comparable to Golang's [os.UserHomeDir()](https://pkg.go.dev/os#UserHomeDir)
or Python's [Path.home()](https://docs.python.org/3/library/pathlib.html#pathlib.Path.home).

## env_home_dir Behavior

This crate provides a simple abstraction to get the user's home directory
in a platform independant way and works on Windows and Unix (Linux/MacOS/BSD/etc).

When the `HOME` or `USERPROFILE` env is set to an empty string, `env_home_dir` returns None.

On platforms other than Unix and Windows (like WASM) that don't implement
a home directory it's effective a no-op and will return `None` if called.

## Differences with `std::env::home_dir`

When the `HOME` or `USERPROFILE` env is set to an empty string
`std::env::home_dir` returns an empty string while `env_home_dir` returns None.

[`std::env::home_dir`](https://doc.rust-lang.org/std/env/fn.home_dir.html)
provided by the standard library may be unexpected on Windows and so has been
[deprecated](https://doc.rust-lang.org/std/env/fn.home_dir.html#deprecation)
since Rust 1.29.0 (Sept 2018).
This is because it incorrectly evaluates the `HOME` environment variable on Windows.
This crate correctly only uses `USERPROFILE` on Windows and only `HOME` on Unix.

[`std::env::home_dir`](https://doc.rust-lang.org/std/env/fn.home_dir.html)
will attempt to use various Platform specific APIs
([GetUserProfileDirectoryW](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getuserprofiledirectoryw)
on Windows and [getpwuid_r](https://linux.die.net/man/3/getpwuid_r) on Unix
as fallback to determine the home directory if the appropriate
`HOME` or `USERPROFILE` environment variables are not set.

This crate intentionally avoids that complexity and only looks at the environment variables.

## Usage

```shell
cargo add env_home
```

Crate exports a single function `env_home_dir` that returns `Option<PathBuf>`

```rust
use env_home::env_home_dir;
fn main() {
    match user_home_dir() {
        Some(path) => println!("User home directory: {}", path.display()),
        None => println!("No home found. HOME/USERPROFILE not set or empty"),
    }
}
```

See the [std::path::PathBuf documentation](https://doc.rust-lang.org/std/path/struct.PathBuf.html)
for more information on how to use `PathBuf` objects: `.push("subdir")`, `.as_path().`, etc.

## Other Notes

Using
[std::env::set_var](https://doc.rust-lang.org/std/env/fn.set_var.html) to alter your environment
at runtime is unsafe in multi-threaded applications. Full stop.
It may result in random panics or undefined behavior. You have have been warned.

Bonus: cargo runs tests in parallel threads by-default, so even if you app is not multi-threaded
if you have tests that invoke `std::env::set_var` be sure to set `RUST_TEST_THREADS=1`
or use `cargo test -- --test-threads=1` or your tests may intermittently panic and fail.

See [rust-lang/rust#27970](https://github.com/rust-lang/rust/issues/27970) and
[Setenv is not Thread Safe and C Doesn't Want to Fix It](https://www.evanjones.ca/setenv-is-not-thread-safe.html)
for more.
