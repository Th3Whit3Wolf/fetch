use cupid;
//use hashbrown::HashMap;
use libc;
use libc::getuid;
use num_cpus;
use systemstat::*;

use sys_info::{cpu_num, cpu_speed, os_release, os_type};
//use std::io::{BufRead, BufReader};

use std::{
    env::{self, var_os},
    ffi::CStr,
    fs::File,
    io::{Error, ErrorKind, Read},
    mem,
    path::Path,
    process::Command,
    ptr, str,
};

fn cpu_parse(s: &str, brand: bool) -> String {
    let s = str::replace(&s, "(TM)", "");
    let s = str::replace(&s, "(tm)", "");
    let s = str::replace(&s, "(R)", "");
    let s = str::replace(&s, "(r)", "");
    let s = str::replace(&s, "CPU", "");
    let s = str::replace(&s, "Processor", "");
    let s = str::replace(&s, "Dual-Core", "");
    let s = str::replace(&s, "Quad-Core", "");
    let s = str::replace(&s, "Six-Core", "");
    let s = str::replace(&s, "Eight-Core", "");
    let s = str::replace(&s, "Compute Cores", "");
    let s = str::replace(&s, "Core ", "");
    let s = str::replace(&s, "AuthenticAMD", "");
    let s = str::replace(&s, "with Radeon", "");
    let s = str::replace(&s, "Graphics", "");
    let s = str::replace(&s, "altivec supported", "");
    let s = str::replace(&s, "FPU", "");
    let s = str::replace(&s, "Chip Revision", "");
    let s = str::replace(&s, "Technologies, Inc", "");
    let s = str::replace(&s, "Core2", "Core 2");

    if brand {
        s.to_string()
    } else {
        let s = str::replace(&s, "Intel ", "");
        let s = str::replace(&s, "AMD ", "");
        let s = str::replace(&s, "Core? Duo ", "");
        let s = str::replace(&s, "Qualcom", "Core 2");
        s.to_string()
    }
}

fn parse_host(s: String) -> String {
    let s = str::replace(&s, "To be filled by O.E.M.", "");
    let s = str::replace(&s, "To Be Filled", "");
    let s = str::replace(&s, "OEM", "");
    let s = str::replace(&s, "Not Applicable", "");
    let s = str::replace(&s, "System Product Name}", "");
    let s = str::replace(&s, "System Version", "");
    let s = str::replace(&s, "Undefined", "");
    let s = str::replace(&s, "Default string", "");
    let s = str::replace(&s, "Not Specified", "");
    let s = str::replace(&s, "Type1ProductConfigId", "");
    let s = str::replace(&s, "INVALID", "");
    let s = str::replace(&s, "�", "");
    let s = str::replace(&s, "Laptop", "");
    s.to_string()
}

// Converts Kilobytes to Megabytes
pub fn megabytes(num: usize) -> String {
    let num = num / 1024;
    let string = num.to_string();
    match num as usize {
        0..=9 => "".to_string() + &string[..1] + " MB",
        10..=99 => "".to_string() + &string[..2] + " MB",
        100..=999 => "".to_string() + &string[..3] + " MB",
        1_000..=9_999 => "".to_string() + &string[..1] + "," + &string[1..4] + " MB",
        10_000..=99_999 => "".to_string() + &string[..2] + "," + &string[2..5] + " MB",
        100_000..=999_999 => "".to_string() + &string[..3] + "," + &string[3..6] + " MB",
        1_000_000..=9_999_999 => {
            "".to_string() + &string[..1] + "," + &string[1..4] + "," + &string[4..7] + " MB"
        }
        10_000_000..=99_999_999 => {
            "".to_string() + &string[..2] + "," + &string[2..5] + "," + &string[5..8] + " MB"
        }
        100_000_000..=999_999_999 => {
            "".to_string() + &string[..3] + "," + &string[3..6] + "," + &string[6..8] + " MB"
        }
        1_000_000_000..=9_999_999_999 => {
            "".to_string()
                + &string[..1]
                + ","
                + &string[1..4]
                + ","
                + &string[4..7]
                + &string[7..10]
                + " MB"
        }
        _ => string + " MB",
    }
}

// Should work for Windows, Linux, MacOS, OpenBSD, FreeBSD, Android
pub fn get_mem(perc: bool) -> String {
    let sys = System::new();
    let mem = sys.memory().expect("failed to get memory information");
    let used = mem.total.as_usize() / 1024 - mem.free.as_usize() / 1024;
    let total = mem.total.as_usize() / 1024;
    let mem_perc = used * 100 / total;

    let total_memory = megabytes(total);
    let used_memory = megabytes(used);

    // Show memory pecentage in output.
    if !perc {
        "Memory: ".to_string() + &used_memory + " / " + &total_memory
    } else {
        "Memory: ".to_string()
            + &used_memory.to_string()
            + " / "
            + &total_memory.to_string()
            + " ("
            + &mem_perc.to_string()
            + "%)"
    }
}

// Toggles
// Cpu Brand
// Cpu Cores
// Cpu Speed
// Cpu temp
// Cpu Name and Speed should work for all x86 and x86_64 cpus
// CPU Num should work for Linux, Windows, MacOS, FreeBSD, DragonflyBSD, Bitrig, NetBSD, NaCL, iOS, Android, Solaris, Fuchsia, Redox, Haiku
// Cpu Temp should work on Windows, Linux, Macos, FreeBSD, OpenBSD, and Android
#[allow(clippy::collapsible_if)]
#[allow(clippy::if_same_then_else)]
pub fn get_cpu(brand: bool, cores: bool, speed: bool, temp: bool) -> String {
    let info = cupid::master().unwrap();
    let name = info.brand_string().unwrap();
    let v: Vec<&str> = name.split(" @").collect();

    let cpu = cpu_parse(v[0], brand);

    let sys = System::new();

    if speed && temp {
        let c_speed = cpu_speed().expect("Error getting cpu speed") as f64 / 1000.0;
        let temp = sys.cpu_temp().expect("Error getting CPU Temperature");
        let t = &format!("{:.0}", temp);

        if cores {
            let cpu = core_count(&cpu);
            format!("CPU: {} @ {:.3}GHz {}°C", cpu, c_speed, t)
        } else {
            format!("CPU: {} @ {:.3}GHz {}°C", cpu, c_speed, t)
        }
    } else if !speed && temp {
        let temp = sys.cpu_temp().expect("Error getting CPU Temperature");
        let t = &format!("{:.1}", temp);

        if cores {
            let cpu = core_count(&cpu);
            "CPU: ".to_string() + &cpu + " " + t + "°C"
        } else {
            "CPU: ".to_string() + &cpu + t + "°C"
        }
    } else if speed && !temp {
        let c_speed = cpu_speed().expect("Error getting cpu speed") as f64 / 1000.0;
        if cores {
            let cpu = core_count(&cpu);
            format!("CPU: {} @ {:.3}GHz", cpu, c_speed)
        } else {
            format!("CPU: {} @ {:.3}GHz", cpu, c_speed)
        }
    } else {
        if cores {
            let cpu = core_count(&cpu);
            "CPU: ".to_string() + &cpu
        } else {
            "CPU: ".to_string() + &cpu
        }
    }
}

fn _old_core_count(name: &str) -> String {
    let logical = num_cpus::get();
    let actual = num_cpus::get_physical();

    if logical == actual {
        let cores = format!("{}({})", name, actual);
        cores
    } else {
        let cores = format!("{}({}/{})", name, actual, logical);
        cores
    }
}

fn core_count(name: &str) -> String {
    let cores = cpu_num();
    let cores = format!(
        "{}({})",
        name,
        cores.expect("Error counting cores").to_string()
    );
    cores
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

// Should work for Windows and Linux
pub fn hostname() -> Option<String> {
    hostname::get_hostname()
}

pub fn make_titlebar(user: &str, hostname: &str) -> String {
    let n = user.chars().count() + hostname.chars().count() + 1;
    let titlebar: String = "-".repeat(n);
    titlebar
}

pub fn get_kernel(shorthand: bool) -> String {
    let kernel = os_release().unwrap();
    kernel.to_string();
    if shorthand {
        "Kernel: ".to_string() + &kernel
    } else {
        let os_type = os_type().unwrap();
        "Kernel: ".to_string() + &os_type.to_string() + " " + &kernel
    }
}

pub fn pathy(path: &str) -> Result<String, Error> {
    if cfg!(target_os = "linux") {
        let mut string = String::with_capacity(11);
        File::open(path)?.read_to_string(&mut string)?;
        string.pop();
        string.trim().to_string();
        Ok(string)
    } else {
        Err(Error::from(ErrorKind::Other))
    }
}

fn android_host() -> String {
    let gpb = Command::new("getprop")
        .arg("ro.product.brand")
        .output()
        .expect("failed to get Brand");

    if gpb.status.success() {
        let gpb = String::from_utf8_lossy(&gpb.stdout);

        let gpm = Command::new("getprop")
            .arg("ro.product.model")
            .output()
            .expect("failed to get Model");

        if gpm.status.success() {
            let gpm = String::from_utf8_lossy(&gpm.stdout);
            "Host: ".to_string() + &gpb.to_string() + &gpm.to_string()
        } else {
            "Host: Unknown".to_string()
        }
    } else {
        "Host: Unknown".to_string()
    }
}

fn mac_host() -> String {
    let hack = Command::new("sh")
        .arg("-c")
        .arg("kextstat | grep -F -e 'FakeSMC' -e 'VirtualSMC'")
        .output()
        .expect("failed to check for Hackers");

    if hack.status.success() {
        let sysctl = Command::new("sysctl")
            .arg("-n")
            .arg("hw.model")
            .output()
            .expect("failed to check for Hackers");
        let sysctl = String::from_utf8_lossy(&sysctl.stdout);
        "Hackintosh (SMBIOS: ".to_string() + &sysctl + ")"
    } else {
        let sysctl = Command::new("sysctl")
            .arg("-n")
            .arg("hw.model")
            .output()
            .expect("failed to check for Hackers");
        let sysctl = String::from_utf8_lossy(&sysctl.stdout);
        sysctl.to_string()
    }
}

fn bsd_host() -> String {
    let sysctl = Command::new("sysctl")
        .arg("-n")
        .arg("hw.model")
        .arg("hw.product")
        .output()
        .expect("failed to check for Hackers");

    if sysctl.status.success() {
        let sysctl = String::from_utf8_lossy(&sysctl.stdout);
        "Host: ".to_string() + &sysctl.to_string()
    } else {
        "Host: Unknown".to_string()
    }
}

pub fn get_host() -> String {
    if cfg!(target_os = "linux") {
        if Path::new("/sys/devices/virtual/dmi/id/product_name").is_file()
            && Path::new("/sys/devices/virtual/dmi/id/product_name").is_file()
        {
            parse_host(
                "Host: ".to_string()
                    + &pathy("/sys/devices/virtual/dmi/id/product_name")
                        .expect("Error reading path")
                    + &pathy("/sys/devices/virtual/dmi/id/product_version")
                        .expect("Error reading path"),
            )
        } else if Path::new("/sys/firmware/devicetree/base/model").is_file() {
            parse_host(
                "Host: ".to_string()
                    + &pathy("/sys/firmware/devicetree/base/model").expect("Error reading path"),
            )
        } else {
            "Host: Unknown".to_string()
        }
    } else if cfg!(target_os = "freebsd")
        || cfg!(target_os = "dragonfly")
        || cfg!(target_os = "bitrig")
        || cfg!(target_os = "netbsd")
    {
        parse_host(bsd_host())
    } else if cfg!(target_os = "mac_os") {
        parse_host(mac_host())
    } else if cfg!(target_os = "android") {
        parse_host(android_host())
    } else {
        "Host: Unknown".to_string()
    }
}

pub fn get_de() -> DesktopEnvironment {
    let de: DesktopEnvironment = detect();
    de
}

#[derive(Debug, PartialEq, Eq)]
pub enum DesktopEnvironment {
    Unknown,
    Cinnamon,
    Enlightenment,
    GNOME,
    KDE,
    LXDE,
    LXQT,
    MacOs,
    Mate,
    Unity,
    Windows,
    XFCE,
}

pub fn detect() -> DesktopEnvironment {
    if cfg!(target_os = "macos") {
        DesktopEnvironment::MacOs
    } else if cfg!(target_os = "windows") {
        DesktopEnvironment::Windows
    } else if cfg!(target_family = "unix") {
        match env::var("XDG_CURRENT_DESKTOP")
            .expect("Failed to parse environment variables")
            .as_str()
        {
            "Cinnamon" => DesktopEnvironment::Cinnamon,
            "ENLIGHTENMENT" => DesktopEnvironment::Enlightenment,
            "GNOME" => DesktopEnvironment::GNOME,
            "KDE" => DesktopEnvironment::KDE,
            "LXDE" => DesktopEnvironment::LXDE,
            "LXQt" => DesktopEnvironment::LXQT,
            "MATE" => DesktopEnvironment::Mate,
            "Unity" => DesktopEnvironment::Unity,
            "X-Cinnamon" => DesktopEnvironment::Cinnamon,
            "XFCE" => DesktopEnvironment::XFCE,
            _ => DesktopEnvironment::Unknown,
        }
    } else {
        DesktopEnvironment::Unknown
    }
}

#[cfg(not(feature = "vulkan"))]
pub fn get_gpu(brand: bool) -> Vec<String> {
    let v: Vec<String> = Vec::with_capacity(2);
    use shells::*;
    let (_code, stdout, _stderr) = sh!("lspci | grep VGA | awk -F ':' '{{print $3}}'");
    let gpu = stdout[..].trim();
    let v = gpu
        .to_string()
        .lines()
        .map(|s| gpu_parse(s.to_string(), brand))
        .collect();
    v
}

#[cfg(not(feature = "vulkan"))]
pub fn gpu_parse(gpu: String, brand: bool) -> String {
    let v: Vec<&str> = Vec::with_capacity(8);
    let v: Vec<&str> = gpu.split_whitespace().collect();
    if v[0] == "Intel" {
        let gpu = v.join(" ");
        let gpu = str::replace(&gpu, "(Mobile)", "");
        let gpu = str::replace(&gpu, "Corporation ", "");
        if !brand {
            str::replace(&gpu, "Intel", "").trim().to_string()
        } else {
            gpu.to_string()
        }
    } else if v[0] == "NVIDIA" {
        let gpu: String = v.join(" ");
        let gpu: String = gpu.split('[').take(2).nth(1).unwrap_or("").to_string();
        let gpu: String = gpu.split(']').take(1).next().unwrap_or("").to_string();

        if !brand {
            gpu
        } else {
            "Nvidia ".to_string() + &gpu
        }
    } else {
        "Unknown".to_string()
    }
}

#[cfg(feature = "vulkan")]
pub fn get_gpu(brand: bool) -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(2);
    use vulkano::instance::Instance;
    use vulkano::instance::InstanceExtensions;
    use vulkano::instance::PhysicalDevice;
    use vulkano::instance::PhysicalDeviceType;
    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();

    for physical_device in PhysicalDevice::enumerate(&instance) {
        v.push(gpu_parse(physical_device, brand));
    }

    fn gpu_parse(gpu: PhysicalDevice, brand: bool) -> String {
        let res: Vec<String> = gpu.name().split(' ').map(|s| s.to_string()).collect();

        let gpu_type = gpu.ty();
        if gpu_type == PhysicalDeviceType::IntegratedGpu {
            if &res[0] == "Intel(R)" {
                let gpu = res.join(" ");
                let gpu = str::replace(&gpu, "(R)", "").to_string();
                let gpu: String = gpu.split('(').take(1).next().unwrap_or("").to_string();
                if !brand {
                    str::replace(&gpu, "Intel ", "")
                } else {
                    gpu
                }
            } else {
                let gpu = res.join(" ");
                if !brand {
                    str::replace(&gpu, "AMD", "").to_string()
                } else {
                    gpu
                }
            }
        } else if gpu_type == PhysicalDeviceType::DiscreteGpu {
            if &res[0] == "GeForce" {
                if !brand {
                    res.join(" ")
                } else {
                    "Nvidia ".to_string() + &res.join(" ")
                }
            } else if !brand {
                res.join(" ")
            } else {
                "AMD ".to_string() + &res.join(" ")
            }
        } else {
            gpu.name()
        }
    }
    v
}

pub enum Shell {
    Windows,
    Bash,
    Tcsh,
    Zsh,
    Ksh,
    Unknown,
}

pub fn get_shell() -> Shell {
    if cfg!(windows) {
        Shell::Windows
    } else {
        if let Some(shell) = var_os("BASH") {
            if shell.to_string_lossy().ends_with("/bash") {
                return Shell::Bash;
            }
        }
        if let Some(zsh) = var_os("ZSH_NAME") {
            if zsh.to_string_lossy() == "zsh" {
                return Shell::Zsh;
            }
        }
        if let Some(shell) = var_os("shell") {
            if shell.to_string_lossy().ends_with("/tcsh") {
                return Shell::Tcsh;
            }
        }
        match var_os("SHELL") {
            None => Shell::Bash,
            Some(oss) => {
                if oss.to_string_lossy().ends_with("/bash") {
                    Shell::Bash
                } else if oss.to_string_lossy().ends_with("/ksh") {
                    Shell::Ksh
                } else if oss.to_string_lossy().ends_with("/zsh") {
                    Shell::Zsh
                } else if oss.to_string_lossy().ends_with("/tcsh") {
                    Shell::Tcsh
                } else {
                    Shell::Bash
                } // many shells support export foo=bar
            }
        }
    }
}

impl Shell {
    pub fn get_name(&self) -> &'static str {
        match *self {
            Shell::Windows => "Windows",
            Shell::Bash => "Bash",
            Shell::Tcsh => "Tcsh",
            Shell::Zsh => "Zsh",
            Shell::Ksh => "Ksh",
            Shell::Unknown => "Unknown",
        }
    }
}

pub fn get_distro() -> String {
    use os_release::OsRelease;
    if Path::new("/etc/os-release").is_file() {
        let release = OsRelease::new().expect("Error Will Robinson");
        release.pretty_name
    } else if Path::new("/etc/redstar-release").is_file() {
        "Red Star OS".to_string()
    } else if Path::new("/etc/siduction-version").is_file() {
        "Siductiond".to_string()
    } else if Path::new("/etc/GoboLinuxVersion").is_file() {
        "GoboLinux".to_string()
    } else if Path::new("/etc/pcbsd-lang").is_file() {
        "PCBSD".to_string()
    } else if Path::new("/etc/trueos-lang").is_file() {
        "TrueOS".to_string()
    } else {
        "Unknown".to_string()
    }
}
