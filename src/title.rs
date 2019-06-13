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

// Should work for Windows and Linux
pub fn hostname() -> Option<String> {
    hostname::get_hostname()
}

pub fn make_titlebar(user: &str, hostname: &str) -> String {
    let n = user.chars().count() + hostname.chars().count() + 1;
    let titlebar: String = "-".repeat(n);
    titlebar
}