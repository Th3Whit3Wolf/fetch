use crate::count_lines;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::result;

fn get_package_count_arch_based() -> String {
    let pacman = Command::new("pacman")
        .arg("-Qq")
        .output()
        .expect("ERROR: Failure Retreiving Pacman Packages");
    let pkgs = count_lines(&String::from_utf8(pacman.stdout).unwrap());
    format!("{} (pacman)", pkgs)
}

#[cfg(feature = "arch")]
fn pacman() -> usize {
    use libalpm_fork::*;
    let alpm = Alpm::new("/", "/var/lib/pacman").unwrap();
    let db = alpm.local_db();
    let paccache = db.pkg_cache();
    paccache.len()
}

fn other_arch_based() -> String {
    let pacman = Command::new("pacman")
        .arg("-Qq")
        .output()
        .expect("ERROR: Failure Retreiving Pacman Packages");
    let s = String::from_utf8(pacman.stdout).unwrap();
    let v: Vec<&str> = s.trim().split('\n').collect();
    v.len().to_string()
}
fn arch_pkg() -> String {
    let pacman = Command::new("pacman")
        .arg("-Qq")
        .output()
        .expect("ERROR: Failure Retreiving Pacman Packages");
    let s = String::from_utf8(pacman.stdout).unwrap();
    s
}

fn another_arch_based() -> String {
    let pacman = Command::new("pacman")
        .arg("-Qq")
        .output()
        .expect("ERROR: Failure Retreiving Pacman Packages");
    let s = String::from_utf8(pacman.stdout).unwrap();
    let v: Vec<&str> = s.trim().split('\n').collect();
    v.len().to_string()
}

pub fn packages() -> std::string::String {
    get_package_count_arch_based()
}

use std::time::Instant;

#[ignore]
#[test]
pub fn pkg() {
    let baseline = Instant::now();
    arch_pkg();
    let t0 = baseline.elapsed();
    let first = Instant::now();
    get_package_count_arch_based();
    let t1 = first.elapsed();
    let second = Instant::now();
    other_arch_based();
    let t2 = second.elapsed();
    println!(
        "\nBaseline Time{:?}\nFast Line Count\nTime: {:?} \n\nOld Line Count\nTime: {:?} \n",
        t0, t1, t2
    );
}

#[cfg(feature = "arch")]
#[test]
pub fn test_pac() {
    let pac = Instant::now();
    let num = pacman();
    let t1 = pac.elapsed();
    println!("lib Alpm Time: {:?}", t1);
    println!("found {} packages", num);
}
