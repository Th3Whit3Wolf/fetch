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
