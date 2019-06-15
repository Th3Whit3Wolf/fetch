extern crate libc;

use crate::get_mem_info;
use crate::Error;
use crate::MemInfo;

use std::fs::File;
use std::io::Read;

#[cfg(target_os = "macos")]
use libc::sysctl;

use std::collections::HashMap;

/// Get memory information.
///
/// On Mac OS X and Windows, the buffers and cached variables of the MemInfo returned are zero.
pub fn mem_info() -> Result<MemInfo, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/meminfo")?.read_to_string(&mut s)?;
        let mut meminfo_hashmap = HashMap::new();
        for line in s.lines() {
            let mut split_line = line.split_whitespace();
            let label = split_line.next();
            let value = split_line.next();
            if value.is_some() && label.is_some() {
                let label = label.unwrap().split(':').nth(0).ok_or(Error::Unknown)?;
                let value = value.unwrap().parse::<u64>().ok().ok_or(Error::Unknown)?;
                meminfo_hashmap.insert(label, value);
            }
        }
        let total = *meminfo_hashmap.get("MemTotal").ok_or(Error::Unknown)?;
        let free = *meminfo_hashmap.get("MemFree").ok_or(Error::Unknown)?;
        let buffers = *meminfo_hashmap.get("Buffers").ok_or(Error::Unknown)?;
        let cached = *meminfo_hashmap.get("Cached").ok_or(Error::Unknown)?;
        let avail = meminfo_hashmap
            .get("MemAvailable")
            .map(|v| v.clone())
            .or_else(|| {
                let sreclaimable = *meminfo_hashmap.get("SReclaimable")?;
                let shmem = *meminfo_hashmap.get("Shmem")?;
                Some(free + buffers + cached + sreclaimable - shmem)
            })
            .ok_or(Error::Unknown)?;
        let swap_total = *meminfo_hashmap.get("SwapTotal").ok_or(Error::Unknown)?;
        let swap_free = *meminfo_hashmap.get("SwapFree").ok_or(Error::Unknown)?;
        Ok(MemInfo {
            total,
            free,
            avail,
            buffers,
            cached,
            swap_total,
            swap_free,
        })
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_mem_info() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}

#[test]
pub fn test_mem_info() {
    let mem = mem_info().unwrap();
    assert!(mem.total > 0);
    println!("mem_info(): {:?}", mem);
}
