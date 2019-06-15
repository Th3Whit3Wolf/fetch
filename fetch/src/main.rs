extern crate lib_fetch;
use lib_fetch::mem_info;

fn main() {
    let mem = mem_info().expect("ERROR: Memory Information Retrieval");
    println!(
        "Memory: {}/{}MB",
        (mem.total - mem.free - mem.cached - mem.buffers) / 1024,
        mem.total / 1024
    );
}
