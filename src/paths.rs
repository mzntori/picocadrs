//! Important paths across different platforms.
//!
//! Mainly the paths of where picoCAD will store project files.

use std::{
    env::consts::OS,
    ffi::OsStr,
};


/// File path where a picoCAD project files are located on Windows systems.
pub const WINDOWS: &str = "%appdata%/Roaming/pico-8/appdata/picocad/";
/// File path where a picoCAD project files are located on OSX systems.
pub const OSX: &str = "~/Library/Application Support/pico-8/appdata/picocad/";
/// File path where a picoCAD project files are located on Linux systems.
pub const LINUX: &str = "~/.lexaloffle/pico-8/appdata/picocad/";


/// Returns the file path where picoCAD project files are located on the system as an `&OsStr`.
/// If the system does not support picoCAD this returns `None`.
///
/// I could only verify that this works on windows, but I don't see why it shouldn't on other systems.
///
/// # Examples
/// ```
/// use std::ffi::OsStr;
/// use picocadrs::paths::{projects_path, WINDOWS};
/// // When target is windows.
/// assert_eq!(projects_path().unwrap(), OsStr::new(WINDOWS));
/// ```
pub fn projects_path() -> Option<&'static OsStr> {
    match OS {
        "windows" => { Some(OsStr::new(WINDOWS)) }
        "macos" => { Some(OsStr::new(OSX)) }
        "linux" => { Some(OsStr::new(LINUX)) }
        &_ => { None }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn path_test_windows() {
        assert_eq!(projects_path().unwrap(), OsStr::new(WINDOWS));
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