use crate::lib::{
    get_cpu, get_de, get_distro, get_gpu, get_host, get_kernel, get_mem, get_shell,
    get_unix_username, hostname, make_titlebar,
};
use crate::packages::packages;
use crate::resolution::get_resolution;
use crate::resolution::xdpyinfo;
use crate::resolution::xrandr;
use crate::resolution::xwininfo;
use crate::term::term;
use crate::uptime::get_uptime;

use std::time::Instant;

#[test]
fn mem() {
    let now = Instant::now();

    // Mem Percent
    let mem = get_mem(true);

    println!("{}", mem);
    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn cpu() {
    let now = Instant::now();

    // Cpu Brand
    // Cpu Cores
    // Cpu Speed
    // Cpu Temp
    let cpu = get_cpu(false, true, false, true);

    println!("{}", cpu);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn uptime() {
    let now = Instant::now();

    let uptime = get_uptime(false);

    println!("{}", uptime);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn user() {
    let now = Instant::now();

    let user = get_unix_username().unwrap();
    println!("{}", user);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn hostn() {
    let now = Instant::now();

    println!("{}", hostname().unwrap());

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn title() {
    let now = Instant::now();

    let user = get_unix_username().unwrap();
    let host = hostname().unwrap();
    println!("{}@{}", user, host);

    println!("\nTime: {:?} \n\n", now.elapsed());
}
#[test]
fn titlebar() {
    let now = Instant::now();
    let user = get_unix_username().unwrap();
    let host = hostname().unwrap();
    let title = make_titlebar(&user, &host);
    println!("{}@{}", user, host);
    println!("{}", title);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn kernel() {
    let now = Instant::now();
    let kernel = get_kernel(true);
    println!("{}", kernel);
    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn distro() {
    let now = Instant::now();

    let distro = get_distro();

    println!("{}", distro);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn hostpc() {
    let now = Instant::now();

    let host = get_host();

    println!("{}", host);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn res() {
    let now = Instant::now();

    let res = get_resolution(false);

    println!("{}", res);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn packager() {
    let now = Instant::now();

    let p = packages(true);

    println!("{}", p);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn max() {
    let now = Instant::now();

    let user = get_unix_username().unwrap();
    let hostname = hostname().unwrap();
    let distro = get_distro();
    let host = get_host();
    let titlebar = make_titlebar(&user, &hostname);
    let kernel = get_kernel(false);
    let uptime = get_uptime(true);
    let packages = packages(true);
    let shell = get_shell().get_name();
    let res = get_resolution(false);
    let de = get_de();

    // Shows Memory Percentage used on true
    let mem = get_mem(true);

    // Cpu Brand - True shows brand
    // Cpu Cores - Need faster way
    // Cpu Speed
    // Cpu Temp

    let cpu = get_cpu(false, true, false, true);

    println!("{}@{}", user, hostname);
    println!("{}", titlebar);
    println!("{}", distro);
    println!("{}", host);
    println!("{}", kernel);
    println!("{}", uptime);
    println!("{}", packages);
    println!("{}", shell);
    println!("Resolution: {}", res);
    println!("DE: {:?}", de);
    //println!("{}", Window Manager);
    //println!("{}", WM Theme);
    //println!("{}", Icons);
    //println!("{}", Terminal);
    //println!("{}", Terminal Font);
    println!("{}", cpu);
    get_gpu(false);
    println!("{}", mem);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn desk() {
    let now = Instant::now();

    let de = get_de();

    println!("DE: {:?}", de);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn gpu() {
    let now = Instant::now();
    get_gpu(false);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn shell() {
    let now = Instant::now();
    let shell = get_shell().get_name();

    println!("{}", shell);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn test() {
    let now = Instant::now();
    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn deep_res1() {
    let now = Instant::now();

    let res = xrandr(false);

    println!("{}", res);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn deep_res2() {
    let now = Instant::now();

    let res = xwininfo(false);

    println!("{}", res);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn deep_res3() {
    let now = Instant::now();

    let res = xdpyinfo(false);

    println!("{}", res);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

#[test]
fn termy() {
    let now = Instant::now();

    let t = &term()[0];

    println!("Terminal: {}", t);

    println!("\nTime: {:?} \n\n", now.elapsed());
}
