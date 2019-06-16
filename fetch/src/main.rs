extern crate lib_fetch;

use lib_fetch::{
    host::host,
    kernel::os_release,
    memory::mem_info,
    os::distro,
    shell::shell,
    title::{get_unix_username, hostname, make_titlebar},
    uptime::uptime,
};

fn main() {
    let mem = mem_info().expect("Error: Failure Retreiving Memory Information");
    let kernel = os_release().expect("Error: Failure Retreiving Kernel Version");
    let user = get_unix_username().expect("ERROR: Failure Retreiving Username");
    let hostname = hostname().expect("ERROR: Failure Retreiving Hostname");
    let title = make_titlebar(&user, &hostname);
    let os = distro();
    let shell = shell().get_name();
    let host = host();
    let uptime = uptime(false);

    println!("{}@{}", user, hostname);
    println!("{}", title);
    println!("OS: {}", os);
    println!("Host: {}", host);
    println!("Kernel: {}", kernel);
    println!("Uptime: {}", uptime);

    println!("Shell: {}", shell);

    println!(
        "Memory: {}MB / {}MB",
        (mem.total - (mem.free + mem.buffers + mem.cached)) / 1024,
        mem.total / 1024
    );
}
