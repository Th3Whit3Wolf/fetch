//use procfs;
//use std::process;
//use std::ptr::null_mut;

use proclist;

/*
pub fn term() {
    let pid = process::id();
    let pids = get_parent_pid(pid);

    for p in pids {
        get_pid_name(p);
    }
    //get_pid_name(pids[1]);
}

fn get_parent_pid(pid: u32) -> Vec<u32> {
    let mut pids: Vec<u32> = Vec::new();
    // ps -o ppid=66393
    let ret = process::Command::new("ps")
        .arg("-o")
        .arg(format!("ppid={}", pid))
        .output();

    if ret.is_err() {
        return pids;
    }

    let output = String::from_utf8_lossy(&ret.unwrap().stdout).to_string();
    for pid in output.split("\n") {
        match pid.parse::<u32>() {
            Ok(p) => pids.push(p),
            Err(_) => break,
        }
    }
    pids
}

fn get_pid_name(pid: u32) {
    // ps -p 66393 -o comm=
    let ret = process::Command::new("ps")
        .arg("-p")
        .arg(format!("{}", pid))
        .arg("-o")
        .arg("comm=")
        .output();

    let name = String::from_utf8_lossy(&ret.unwrap().stdout).to_string();

    println!("{:?}", name);
}

pub fn term2() -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(3);

    for prc in procfs::all_processes() {
        match prc.stat.comm.trim() as &str {
            "konsole" => v.push(String::from("Konsole")),
            _ => continue,
        }
    }
    v
}
*/

pub fn term() -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(3);
    for process_info in proclist::iterate_processes_info().filter_map(|r| r.ok()) {
        match &process_info.name.trim() as &str {
            "konsole" => v.push(String::from("Konsole")),
            _ => continue,
        }
    }
    if v.is_empty() {
        v.push(String::from("Unknown"))
    }
    v
}
