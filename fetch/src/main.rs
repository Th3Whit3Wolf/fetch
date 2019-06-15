extern crate lib_fetch;

use lib_fetch::kernel::os_release;
use lib_fetch::memory::mem_info;
use lib_fetch::title::{get_unix_username, hostname, make_titlebar};

fn main() {
    let mem = mem_info().expect("Error: Failure Retreiving Memory Information");
    let kernel = os_release().expect("Error: Failure Retreiving Kernel Version");
    let user = get_unix_username().expect("ERROR: Failure Retreiving Username");
    let hostname = hostname().expect("ERROR: Failure Retreiving Hostname");
    let title = make_titlebar(&user, &hostname);
    println!("{}@{}", user, hostname);
    println!("{}", title);
    println!("Kernel: {}", kernel);

    println!(
        "Memory: {}MB / {}MB",
        (mem.total - mem.free - mem.buffers - mem.cached) / 1024,
        mem.total / 1024
    );
}
