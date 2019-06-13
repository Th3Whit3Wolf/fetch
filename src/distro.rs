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
