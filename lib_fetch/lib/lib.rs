//! #Introduction
//! This crate focuses on geting system information.
//!
//! For now it mostly supports Mac OS X and Windows.
//! It will always support Linux
//! And now it can get information of kernel/cpu/memory/disk/load/hostname and so on.
//!

pub mod de_wm;
pub mod host;
pub mod kernel;
pub mod memory;
pub mod os;
pub mod package;
pub mod shell;
pub mod title;
pub mod uptime;

#[macro_use]
extern crate lazy_static;
extern crate libc;
//extern crate test as other_test;

use std::cmp;

use std::{
    ffi, fmt,
    fs::File,
    io::{self, Read},
    os::raw::c_char,
};

#[cfg(target_os = "macos")]
use libc::sysctl;

#[cfg(target_pointer_width = "16")]
const USIZE_BYTES: usize = 2;
#[cfg(target_pointer_width = "32")]
const USIZE_BYTES: usize = 4;
#[cfg(target_pointer_width = "64")]
const USIZE_BYTES: usize = 8;
const LO: usize = ::std::usize::MAX / 255;
const HI: usize = LO * 128;
const REP_NEWLINE: usize = b'\n' as usize * LO;

/// System load average value.
#[repr(C)]
#[derive(Debug)]
pub struct LoadAvg {
    /// Average load within one minite.
    pub one: f64,
    /// Average load within five minites.
    pub five: f64,
    /// Average load within fifteen minites.
    pub fifteen: f64,
}

/// System memory information.
#[repr(C)]
#[derive(Debug)]
pub struct MemInfo {
    /// Total physical memory.
    pub total: u64,
    pub free: u64,
    pub avail: u64,

    pub buffers: u64,
    pub cached: u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free: u64,
}

/// Disk information.
#[repr(C)]
#[derive(Debug)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64,
}

/// Error types
#[derive(Debug)]
pub enum Error {
    UnsupportedSystem,
    ExecFailed(io::Error),
    IO(io::Error),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnsupportedSystem => write!(fmt, "System is not supported"),
            ExecFailed(ref e) => write!(fmt, "Execution failed: {}", e),
            IO(ref e) => write!(fmt, "IO error: {}", e),
            Unknown => write!(fmt, "An unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnsupportedSystem => "unsupported system",
            ExecFailed(_) => "execution failed",
            IO(_) => "io error",
            Unknown => "unknown error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        use self::Error::*;
        match *self {
            UnsupportedSystem => None,
            ExecFailed(ref e) => Some(e),
            IO(ref e) => Some(e),
            Unknown => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

extern "C" {
    fn get_os_type() -> *const i8;
    fn get_os_release() -> *const i8;

    fn get_cpu_num() -> u32;
    fn get_cpu_speed() -> u64;

    fn get_loadavg() -> LoadAvg;
    fn get_proc_total() -> u64;

    fn get_mem_info() -> MemInfo;
    fn get_disk_info() -> DiskInfo;
}

/// Get operation system type.
///
/// Such as "Linux", "Darwin", "Windows".
pub fn os_type() -> Result<String, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/sys/kernel/ostype")?.read_to_string(&mut s)?;
        s.pop(); // pop '\n'
        Ok(s)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        let typ = unsafe { ffi::CStr::from_ptr(get_os_type() as *const c_char).to_bytes() };
        Ok(String::from_utf8_lossy(typ).into_owned())
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get cpu num quantity.
///
/// Notice, it returns the logical cpu quantity.
pub fn cpu_num() -> Result<u32, Error> {
    if cfg!(unix) || cfg!(windows) {
        unsafe { Ok(get_cpu_num()) }
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    if cfg!(target_os = "linux") {
        // /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_cur_freq
        let mut s = String::new();
        File::open("/proc/cpuinfo")?.read_to_string(&mut s)?;

        let mut find_cpu_mhz = s.split('\n').find(|line| line.starts_with("cpu MHz"));
        match find_cpu_mhz {
            None => find_cpu_mhz = s.split('\n').find(|line| line.starts_with("BogoMIPS")),
            _ => {}
        }

        find_cpu_mhz
            .and_then(|line| line.split(':').last())
            .and_then(|val| val.trim().parse::<f64>().ok())
            .map(|speed| speed as u64)
            .ok_or(Error::Unknown)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        unsafe { Ok(get_cpu_speed()) }
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get system load average value.
///
/// Notice, on windows, one/five/fifteen of the LoadAvg returned are the current load.
pub fn loadavg() -> Result<LoadAvg, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/loadavg")?.read_to_string(&mut s)?;
        let loads = s
            .trim()
            .split(' ')
            .take(3)
            .map(|val| val.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        Ok(LoadAvg {
            one: loads[0],
            five: loads[1],
            fifteen: loads[2],
        })
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_loadavg() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get current processes quantity.
///
/// Notice, it temporarily does not support Windows.
pub fn proc_total() -> Result<u64, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/loadavg")?.read_to_string(&mut s)?;
        s.split(' ')
            .nth(3)
            .and_then(|val| val.split('/').last())
            .and_then(|val| val.parse::<u64>().ok())
            .ok_or(Error::Unknown)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_proc_total() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get disk information.
///
/// Notice, it just calculate current disk on Windows.
pub fn disk_info() -> Result<DiskInfo, Error> {
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_disk_info() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}

fn count_lines(s: &str) -> usize {
    fn mask_zero(x: usize) -> usize {
        ((x.wrapping_sub(LO)) & !x & HI) >> 7
    }
    let text = s.as_bytes();
    let (ptr, len) = (text.as_ptr(), text.len());

    let align = (ptr as usize) & (USIZE_BYTES - 1);
    let mut offset;
    let mut count;
    if align > 0 {
        offset = cmp::min(USIZE_BYTES - align, len);
        count = text[..offset].iter().filter(|b| **b == b'\n').count();
    } else {
        offset = 0;
        count = 0;
    }
    while offset + 8 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);
            let x2 = *(ptr.offset((offset + USIZE_BYTES * 2) as isize) as *const usize);
            let x3 = *(ptr.offset((offset + USIZE_BYTES * 3) as isize) as *const usize);
            let x4 = *(ptr.offset((offset + USIZE_BYTES * 4) as isize) as *const usize);
            let x5 = *(ptr.offset((offset + USIZE_BYTES * 5) as isize) as *const usize);
            let x6 = *(ptr.offset((offset + USIZE_BYTES * 6) as isize) as *const usize);
            let x7 = *(ptr.offset((offset + USIZE_BYTES * 7) as isize) as *const usize);

            count += ((mask_zero(x0 ^ REP_NEWLINE)
                + mask_zero(x1 ^ REP_NEWLINE)
                + mask_zero(x2 ^ REP_NEWLINE)
                + mask_zero(x3 ^ REP_NEWLINE))
                + (mask_zero(x4 ^ REP_NEWLINE)
                    + mask_zero(x5 ^ REP_NEWLINE)
                    + mask_zero(x6 ^ REP_NEWLINE)
                    + mask_zero(x7 ^ REP_NEWLINE)))
            .wrapping_mul(LO)
                >> ((USIZE_BYTES - 1) * 8);
        }
        offset += USIZE_BYTES * 8;
    }
    while offset + 4 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);
            let x2 = *(ptr.offset((offset + USIZE_BYTES * 2) as isize) as *const usize);
            let x3 = *(ptr.offset((offset + USIZE_BYTES * 3) as isize) as *const usize);

            count += (mask_zero(x0 ^ REP_NEWLINE)
                + mask_zero(x1 ^ REP_NEWLINE)
                + mask_zero(x2 ^ REP_NEWLINE)
                + mask_zero(x3 ^ REP_NEWLINE))
            .wrapping_mul(LO)
                >> ((USIZE_BYTES - 1) * 8)
        }
        offset += USIZE_BYTES * 4;
    }
    while offset + 2 * USIZE_BYTES <= len {
        unsafe {
            let x0 = *(ptr.offset(offset as isize) as *const usize);
            let x1 = *(ptr.offset((offset + USIZE_BYTES) as isize) as *const usize);

            count += (mask_zero(x0 ^ REP_NEWLINE) + mask_zero(x1 ^ REP_NEWLINE)).wrapping_mul(LO)
                >> ((USIZE_BYTES - 1) * 8)
        }
        offset += USIZE_BYTES * 2;
    }
    while offset + USIZE_BYTES <= len {
        let x0 = unsafe { *(ptr.offset(offset as isize) as *const usize) };
        count += mask_zero(x0 ^ REP_NEWLINE).wrapping_mul(LO) >> ((USIZE_BYTES - 1) * 8);
        offset += USIZE_BYTES;
    }
    count + text[offset..].iter().filter(|b| **b == b'\n').count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_os_type() {
        let typ = os_type().unwrap();
        assert!(typ.len() > 0);
        println!("os_type(): {}", typ);
    }

    #[test]
    pub fn test_cpu_num() {
        let num = cpu_num().unwrap();
        assert!(num > 0);
        println!("cpu_num(): {}", num);
    }

    #[test]
    pub fn test_cpu_speed() {
        let speed = cpu_speed().unwrap();
        assert!(speed > 0);
        println!("cpu_speed(): {}", speed);
    }

    #[test]
    pub fn test_loadavg() {
        let load = loadavg().unwrap();
        println!("loadavg(): {:?}", load);
    }

    #[test]
    pub fn test_proc_total() {
        let procs = proc_total().unwrap();
        assert!(procs > 0);
        println!("proc_total(): {}", procs);
    }

    #[test]
    pub fn test_disk_info() {
        let info = disk_info().unwrap();
        println!("disk_info(): {:?}", info);
    }
}
