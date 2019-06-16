extern crate lib_fetch;

use lib_fetch::{
    de_wm::{get_de, DesktopEnvironment},
    host::host,
    kernel::os_release,
    memory::mem_info,
    os::distro,
    package::packages,
    shell::shell,
    title::{get_unix_username, hostname, make_titlebar},
    uptime::uptime,
};

fn main() {
    let mem: lib_fetch::MemInfo = mem_info().expect("Error: Failure Retreiving Memory Information");
    let kernel: String = os_release().expect("Error: Failure Retreiving Kernel Version");
    let user: String = get_unix_username().expect("ERROR: Failure Retreiving Username");
    let hostname: String = hostname().expect("ERROR: Failure Retreiving Hostname");
    let title: String = make_titlebar(&user, &hostname);
    let os: String = distro();
    let shell: &'static str = shell().get_name();
    let host: String = host();
    let uptime: String = uptime(false);
    let de: DesktopEnvironment = get_de();
    let pkg: String = packages();

    println!("{}@{}", user, hostname);
    println!("{}", title);
    println!("OS: {}", os);
    println!("Host: {}", host);
    println!("Kernel: {}", kernel);
    println!("Uptime: {}", uptime);
    println!("DE: {:?}", de);
    println!("Packages: {}", pkg);

    println!("Shell: {}", shell);

    println!(
        "Memory: {}MB / {}MB",
        (mem.total - (mem.free + mem.buffers + mem.cached)) / 1024,
        mem.total / 1024
    );
}
