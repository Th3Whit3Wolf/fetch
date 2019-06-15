extern crate libc;

use crate::get_os_release;
use crate::Error;
use std::ffi;

use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;

#[cfg(target_os = "macos")]
use libc::sysctl;

/// Get operation system release version.
///
/// Such as "3.19.0-gentoo"
pub fn os_release() -> Result<String, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/sys/kernel/osrelease")?.read_to_string(&mut s)?;
        Ok(s.trim().to_string())
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        let typ = unsafe { ffi::CStr::from_ptr(get_os_release() as *const c_char).to_bytes() };
        Ok(String::from_utf8_lossy(typ).into_owned())
    } else {
        Err(Error::UnsupportedSystem)
    }
}

#[test]
pub fn test_os_release() {
    let release = os_release().unwrap();
    assert!(release.len() > 0);
    println!("os_release(): {}", release);
}
