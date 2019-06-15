use crate::Error;
use libc::getuid;

use std::{ffi::CStr, mem, ptr};

/// Get hostname.
pub fn hostname() -> Result<String, Error> {
    use std::process::Command;
    if cfg!(unix) {
        Command::new("hostname")
            .output()
            .map_err(Error::ExecFailed)
            .map(|output| String::from_utf8(output.stdout).unwrap().trim().to_string())
    } else if cfg!(windows) {
        Command::new("hostname")
            .output()
            .map_err(Error::ExecFailed)
            .map(|output| String::from_utf8(output.stdout).unwrap().trim().to_string())
    } else {
        Err(Error::UnsupportedSystem)
    }
}

//Should work for unix
pub fn get_unix_username() -> Option<String> {
    unsafe {
        let uid = getuid();
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();

        match libc::getpwuid_r(
            uid,
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity() as libc::size_t,
            &mut result,
        ) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_name as *const _;
                let username = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
                Some(username)
            }
            _ => None,
        }
    }
}

pub fn make_titlebar(user: &str, hostname: &str) -> String {
    let n = user.chars().count() + hostname.chars().count() + 1;
    let titlebar: String = "-".repeat(n);
    titlebar
}

#[test]
pub fn test_hostname() {
    let host = hostname().unwrap();
    assert!(host.len() > 0);
    println!("hostname(): {}", host);
}

#[test]
pub fn test_username() {
    let user = get_unix_username().unwrap();
    assert!(user.len() > 0);
    println!("username(): {}", user);
}

#[test]
pub fn test_titlebar() {
    let host = hostname().unwrap();
    assert!(host.len() > 0);
    let user = get_unix_username().unwrap();
    assert!(user.len() > 0);
    let title = make_titlebar(&user, &host);
    println!("title(): {}", title);
}
