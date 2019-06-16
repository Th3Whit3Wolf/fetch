use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
    path::Path,
    process::Command,
    str,
};

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

pub fn host() -> String {
    if cfg!(target_os = "linux") {
        if Path::new("/sys/devices/virtual/dmi/id/product_name").is_file()
            && Path::new("/sys/devices/virtual/dmi/id/product_name").is_file()
        {
            parse_host(
                pathy("/sys/devices/virtual/dmi/id/product_name").expect("Error reading path")
                    + &pathy("/sys/devices/virtual/dmi/id/product_version")
                        .expect("Error reading path"),
            )
        } else if Path::new("/sys/firmware/devicetree/base/model").is_file() {
            parse_host(pathy("/sys/firmware/devicetree/base/model").expect("Error reading path"))
        } else {
            String::from("Unknown")
        }
    } else if cfg!(target_os = "freebsd")
        || cfg!(target_os = "dragonfly")
        || cfg!(target_os = "bitrig")
        || cfg!(target_os = "netbsd")
    {
        let sysctl = Command::new("sysctl")
            .arg("-n")
            .arg("hw.model")
            .arg("hw.product")
            .output()
            .expect("failed to check for Hackers");

        if sysctl.status.success() {
            let sysctl = String::from_utf8_lossy(&sysctl.stdout);
            parse_host(sysctl.to_string())
        } else {
            String::from("Unknown")
        }
    } else if cfg!(target_os = "mac_os") {
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
            parse_host("Hackintosh (SMBIOS: ".to_string() + &sysctl + ")")
        } else {
            let sysctl = Command::new("sysctl")
                .arg("-n")
                .arg("hw.model")
                .output()
                .expect("failed to check for Hackers");
            let sysctl = String::from_utf8_lossy(&sysctl.stdout);
            parse_host(sysctl.to_string())
        }
    } else if cfg!(target_os = "android") {
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
                parse_host(gpb.to_string() + &gpm.to_string())
            } else {
                String::from("Unknown")
            }
        } else {
            String::from("Unknown")
        }
    } else {
        String::from("Unknown")
    }
}
