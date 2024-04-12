//! Important paths across different platforms.
//!
//! Mainly the paths of where picoCAD will store project files.

use std::env::consts::OS;
use std::ffi::OsString;

/// File path where a picoCAD project files are located on Windows systems relative to home directory.
pub const WINDOWS: &str = r#"\AppData\Roaming\pico-8\appdata\picocad\"#;
/// File path where a picoCAD project files are located on OSX systems relative to home directory.
pub const OSX: &str = "/Library/Application Support/pico-8/appdata/picocad/";
/// File path where a picoCAD project files are located on Linux systems relative to home directory.
pub const LINUX: &str = "/.lexaloffle/pico-8/appdata/picocad/";

/// Returns the file path where picoCAD project files are located on the system as an [`OsString`](OsString).
/// If there is no home directory found this returns [`None`].
/// If this returns [`None`] when it shouldn't check
/// [`this`](https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.new)
/// methods documentation, which this function relies on.
///
/// I could verify that this works on windows, but I don't see why it shouldn't on macOS or linux.
pub fn projects_path() -> Option<OsString> {
    return if let Some(user_dirs) = directories::UserDirs::new() {
        let mut path = user_dirs.home_dir().as_os_str().to_os_string();
        path.push(match OS {
            "windows" => WINDOWS,
            "linux" => LINUX,
            "macos" => OSX,
            &_ => "",
        });
        Some(path)
    } else {
        None
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn path_test_windows() {
        let user_dirs = directories::UserDirs::new().unwrap();
        let mut path = user_dirs.home_dir().as_os_str().to_os_string();
        path.push(WINDOWS);

        assert_eq!(path, projects_path().unwrap())
    }

    #[test]
    #[ignore]
    fn path_test_linux() {
        assert_eq!(projects_path().unwrap(), OsStr::new(LINUX));
    }

    #[test]
    #[ignore]
    fn path_test_macos() {
        assert_eq!(projects_path().unwrap(), OsStr::new(OSX));
    }
}
