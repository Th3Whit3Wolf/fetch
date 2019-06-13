/*
use cupid;
//use hashbrown::HashMap;
use libc;
use libc::getuid;
use num_cpus;
use systemstat::*;

use sys_info::{cpu_num, cpu_speed, os_release, os_type};
//use std::io::{BufRead, BufReader};
*/

//#[macro_use]
//extern crate lazy_static;
//#[cfg(any(target_os = "linux", target_os = "android"))]
//#[macro_use]
//extern crate nom;



//pub use self::data::*;

pub use bytesize::ByteSize;
pub use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use libc::{
    c_char, c_int, c_long, c_schar, c_uint, c_ulong, c_ushort, freeifaddrs, getifaddrs, ifaddrs,
    sockaddr, sockaddr_in6, statvfs, AF_INET, AF_INET6,
};
use nom::{digit, is_space, not_line_ending, space};
pub use std::collections::BTreeMap;
use std::io::Read;
use std::ops::Sub;
use std::path::Path;
use std::str;
pub use std::time::Duration;
use std::{ffi, fs, io, mem, path, ptr};



fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_string(&mut s))
        .map(|_| s)
}

fn value_from_file<T: str::FromStr>(path: &str) -> io::Result<T> {
    read_file(path)?
        .trim_end_matches('\n')
        .parse()
        .and_then(Ok)
        .or_else(|_| {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("File: \"{}\" doesn't contain an int value", &path),
            ))
        })
}


