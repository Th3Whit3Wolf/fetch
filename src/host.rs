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