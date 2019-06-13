use std::time::Instant;

mod lib;
mod packages;
mod resolution;
mod term;
#[cfg(test)]
mod test;
mod uptime;

fn main() {
    let now = Instant::now();
    let gpus: Vec<String> = Vec::with_capacity(2);
    let gpus = lib::get_gpu(false);
    let user = lib::get_unix_username().unwrap();
    let hostname = lib::hostname().unwrap();
    let titlebar = lib::make_titlebar(&user, &hostname);
    //let res = resolution::get_resolution(true);
    let distro = lib::get_distro();
    let host = lib::get_host();
    let kernel = lib::get_kernel(true);
    let uptime = uptime::get_uptime(false);
    //let packages = packages::packages(true);
    let shell = lib::get_shell().get_name();
    let de = lib::get_de();
    //let terminal = &term::term()[0];

    // Shows Memory Percentage used on true
    let mem = lib::get_mem(false);

    // Cpu Brand - True shows brand
    // Cpu Cores - Need faster way
    // Cpu Speed
    // Cpu Temp

    let cpu = lib::get_cpu(true, false, false, false);

    println!("{}@{}", user, hostname);
    println!("{}", titlebar);
    println!("Distro: {}", distro);
    println!("{}", host);
    println!("{}", kernel);
    println!("{}", uptime);
    //println!("{}", packages);
    println!("Shell: {}", shell);
    //println!("Resolution: {}", res);
    println!("DE: {:?}", de);
    //println!("{}", Window Manager);
    //println!("{}", WM Theme);
    //println!("{}", Icons);
    //println!("Terminal: {}", terminal);
    //println!("{}", Terminal Font);
    println!("{}", cpu);
    for i in gpus {
        println!("GPU: {}", i)
    }
    println!("{}", mem);

    println!("\nTime: {:?} \n\n", now.elapsed());
}

/*
    List of outputs
        title
        titlebar
        OS      --os_arch
            on:  'Arch Linux x86_64'
            off: 'Arch Linux'

        Host
        Kernel	--kernel_shorthand
            on:  '4.8.9-1-ARCH'
            off: 'Linux 4.8.9-1-ARCH'

        Uptime [x]  --uptime_shorthand
            on:   '2 days, 10 hours, 3 mins'
            off: '2d 10h 3m'

        Packages	    --package_managers
            on:   '998 (pacman), 8 (flatpak), 4 (snap)'
            off:  '908'
        Shell
            With version
        Resolution
        Desktop Environment
        Window Manager
        Window Manager theme
        Theme
        Icons
        Terminal
        Terminal Font
        CPU [x] .. except for Temperature
        GPU
        Memory [x]  --memory_percent
            on:   '1801MiB / 7881MiB (22%)'
            off:  '1801MiB / 7881MiB'
        Gpu Driver
        Cpu Usage
        Disk
        Battery
        Font
        Song
        Music Player
        Local IP
        Public IP
        Users
        Locale
        Color Collumns
*/

/* List of Shells ()    ->  chsh -s full-path-to-shell
        Bash
        Dash
        KornShell(ksh)
        Zsh
        tcsh
        Elvish
        fish
        Nash
        Oh
        powershell
        rc
        xonsh
        ion
*/
