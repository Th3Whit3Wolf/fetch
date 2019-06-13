mod lib;

/// Parse a `/proc/meminfo` line into (key, ByteSize)
named!(
    proc_meminfo_line<(String, ByteSize)>,
    complete!(do_parse!(
        key: flat_map!(take_until!(":"), parse_to!(String)) >>
        tag!(":") >>
        value: usize_s >>
        ws!(tag!("kB")) >>
        ((key, ByteSize::kib(value as u64)))
    ))
);

/// Optionally parse a `/proc/meminfo` line`
named!(
    proc_meminfo_line_opt<Option<(String, ByteSize)>>,
    opt!(proc_meminfo_line)
);

/// Parse `/proc/meminfo` into a hashmap
named!(
    proc_meminfo<BTreeMap<String, ByteSize>>,
    fold_many0!(
        ws!(flat_map!(not_line_ending, proc_meminfo_line_opt)),
        BTreeMap::new(),
        |mut map: BTreeMap<String, ByteSize>, opt| {
            if let Some((key, val)) = opt {
                map.insert(key, val);
            }
            map
        }
    )
);

/// Get memory statistics
fn memory_stats() -> io::Result<BTreeMap<String, ByteSize>> {
    read_file("/proc/meminfo").and_then(|data| {
        proc_meminfo(data.as_bytes()).to_result().map_err(|err| {
            io::Error::new(io::ErrorKind::InvalidData, err.to_string())
        })
    })
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub struct PlatformMemory {
    pub load: u32,
    pub total_phys: ByteSize,
    pub avail_phys: ByteSize,
    pub total_pagefile: ByteSize,
    pub avail_pagefile: ByteSize,
    pub total_virt: ByteSize,
    pub avail_virt: ByteSize,
    pub avail_ext: ByteSize,
}

#[cfg(target_os = "freebsd")]
#[derive(Debug, Clone)]
pub struct PlatformMemory {
    pub active: ByteSize,
    pub inactive: ByteSize,
    pub wired: ByteSize,
    pub cache: ByteSize,
    pub zfs_arc: ByteSize,
    pub free: ByteSize,
}

#[cfg(target_os = "openbsd")]
#[derive(Debug, Clone)]
pub struct PlatformMemory {
    pub active: ByteSize,
    pub inactive: ByteSize,
    pub wired: ByteSize,
    pub cache: ByteSize,
    pub free: ByteSize,
    pub paging: ByteSize,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone)]
pub struct PlatformMemory {
    pub active: ByteSize,
    pub inactive: ByteSize,
    pub wired: ByteSize,
    pub cache: ByteSize,
    pub free: ByteSize,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[derive(Debug, Clone)]
pub struct PlatformMemory {
    pub meminfo: BTreeMap<String, ByteSize>,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub total: ByteSize,
    pub free: ByteSize,
    pub platform_memory: PlatformMemory,
}

#[derive(Debug, Clone)]
pub struct BatteryLife {
    pub remaining_capacity: f32,
    pub remaining_time: Duration,
}

fn memory(&self) -> io::Result<Memory> {
        memory_stats()
            .or_else(|_| {
                // If there's no procfs, e.g. in a chroot without mounting it or something
                let mut meminfo = BTreeMap::new();
                let mut info: sysinfo = unsafe { mem::zeroed() };
                unsafe { sysinfo(&mut info) };
                let unit = info.mem_unit as u64;
                meminfo.insert(
                    "MemTotal".to_owned(),
                    ByteSize::b(info.totalram as u64 * unit),
                );
                meminfo.insert(
                    "MemFree".to_owned(),
                    ByteSize::b(info.freeram as u64 * unit),
                );
                meminfo.insert(
                    "Shmem".to_owned(),
                    ByteSize::b(info.sharedram as u64 * unit),
                );
                meminfo.insert(
                    "Buffers".to_owned(),
                    ByteSize::b(info.bufferram as u64 * unit),
                );
                meminfo.insert(
                    "SwapTotal".to_owned(),
                    ByteSize::b(info.totalswap as u64 * unit),
                );
                meminfo.insert(
                    "SwapFree".to_owned(),
                    ByteSize::b(info.freeswap as u64 * unit),
                );
                Ok(meminfo)
            })
            .map(|meminfo| {
                Memory {
                    total: meminfo.get("MemTotal").map(|x| x.clone()).unwrap_or(
                        ByteSize::b(0),
                    ),
                    free: meminfo.get("MemFree").map(|x| x.clone()).unwrap_or(
                        ByteSize::b(0),
                    ) +
                        meminfo.get("Buffers").map(|x| x.clone()).unwrap_or(
                            ByteSize::b(0),
                        ) +
                        meminfo.get("Cached").map(|x| x.clone()).unwrap_or(
                            ByteSize::b(0),
                        ) +
                        ByteSize::b(
                            meminfo.get("SReclaimable").map(|x| x.clone()).unwrap_or(
                                ByteSize::b(0),
                            ).as_u64() -
                            meminfo.get("Shmem").map(|x| x.clone()).unwrap_or(
                                ByteSize::b(0),
                            ).as_u64()
                        ),
                    platform_memory: PlatformMemory { meminfo: meminfo },
                }
            })
    }