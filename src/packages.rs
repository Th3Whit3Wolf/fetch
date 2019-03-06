use std::{fs, path::Path, process::Command, str};

/* List of Package Managers

    Linux
    -----
    dpkg
    rpm
    xbps-query
    apk
    opkg
    pacman-g2
    lvu
    tce-status
    pkg_info
    tazpkg
    sorcery
    alps
    butch

    BSD
    ---
    pkg

    Other
    -----
    flatpack
    spm
    puyo


*/
#[cfg(feature = "arch")]
fn pacman() -> usize {
    use libalpm_fork::*;
    let alpm = Alpm::new("/", "/var/lib/pacman").unwrap();
    let db = alpm.local_db();
    let paccache = db.pkg_cache();
    paccache.len()
}

#[cfg(feature = "deb")]
fn apt() -> usize {
    use apt_pkg_native;
    let mut cache = apt_pkg_native::Cache::get_singleton();
    cache.iter().map(|_| ()).count()
}

#[cfg(feature = "rpm")]
fn rpm() -> usize {
    use librpm::*;
    librpm::config::read_file(None).unwrap();
    let cache = installed_packages();
    let total_packages = cache.map(|_| ()).count();
}

fn yum() -> usize {
    let yum = Command::new("yum")
        .arg("list")
        .arg("installed")
        .output()
        .expect("failed to execute process");

    let s = String::from_utf8(yum.stdout).unwrap();
    let v: Vec<&str> = s.trim().split('\n').collect();
    v.len()
}

fn rpm() -> usize {
    let rpm = Command::new("rpm")
        .arg("-qa")
        .output()
        .expect("failed to execute process");

    let s = String::from_utf8(rpm.stdout).unwrap();
    let v: Vec<&str> = s.trim().split('\n').collect();
    v.len()
}

#[cfg(feature = "arch")]
pub fn packages(package_manager: bool) -> String {
    let mut c = Vec::with_capacity(3);
    let mut s = Vec::with_capacity(3);

    if usable("/usr/bin/pacman") {
        if !package_manager {
            c.push(pacman())
        } else {
            s.push(pacman().to_string() + " (pacman)")
        }
    }

    if !package_manager {
        let mut sum: usize = 0;
        if s.is_empty() {
            "Packages: Unknown".to_string()
        } else {
            for i in c {
                sum += i;
            }
            let packages: String = "Packages: ".to_string() + &sum.to_string();
            packages
        }
    } else {
        if s.is_empty() {
            "Packages: Unknown".to_string()
        } else {
            let join = s.join(", ");
            let packages: String = "Packages: ".to_string() + &join;
            packages
        }
    }
}

#[cfg(feature = "rpm")]
pub fn packages(package_manager: bool) -> String {
    let mut c = Vec::with_capacity(3);
    let mut s = Vec::with_capacity(3);

    if usable("/usr/bin/rpm") {
        if !package_manager {
            c.push(rpm())
        } else {
            s.push(rpm().to_string() + " (rpm)")
        }
    }

    if !package_manager {
        let mut sum: usize = 0;
        if s.is_empty() {
            "Packages: Unknown".to_string()
        } else {
            for i in c {
                sum += i;
            }
            let packages: String = "Packages: ".to_string() + &sum.to_string();
            packages
        }
    } else {
        if s.is_empty() {
            "Packages: Unknown".to_string()
        } else {
            let join = s.join(", ");
            let packages: String = "Packages: ".to_string() + &join;
            packages
        }
    }
}

#[cfg(not(any(feature = "arch", feature = "deb", feature = "rpm")))]
pub fn packages(package_manager: bool) -> String {
    let mut c = Vec::with_capacity(3);
    let mut s = Vec::with_capacity(3);

    if usable("/usr/bin/yum") {
        if !package_manager {
            c.push(yum())
        } else {
            s.push(yum().to_string() + " (yum)")
        }
    } else {
        if usable("/usr/bin/rpm") {
            if !package_manager {
                c.push(rpm())
            } else {
                s.push(rpm().to_string() + " (rpm)")
            }
        }
    }

    if !package_manager {
        let mut sum: usize = 0;
        if s.is_empty() {
            "Packages: Unknown".to_string()
        } else {
            for i in c {
                sum += i;
            }
            let packages: String = "Packages: ".to_string() + &sum.to_string();
            packages
        }
    } else {
        if s.is_empty() {
            "Packages: Unknown".to_string()
        } else {
            let join = s.join(", ");
            let packages: String = "Packages: ".to_string() + &join;
            packages
        }
    }
}

fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_executable()
}

pub trait IsExecutable {
    // Returns `true` if there is a file at the given path and it is
    // executable. Returns `false` otherwise.
    fn is_executable(&self) -> bool;
}

impl IsExecutable for Path {
    fn is_executable(&self) -> bool {
        fs::metadata(self)
            .ok()
            .map_or(false, |meta| meta.permissions().is_executable())
    }
}

impl IsExecutable for fs::Permissions {
    fn is_executable(&self) -> bool {
        use std::os::unix::fs::PermissionsExt;
        self.mode() & 0o111 != 0
    }
}

fn path_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}

pub fn usable(path: &str) -> bool {
    let new = Path::new(path);
    if path_exists(new) {
        is_executable(new)
    } else {
        false
    }
}
