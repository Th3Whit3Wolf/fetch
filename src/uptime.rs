use std::fmt::Write;
use std::time::Duration;
// Platforms supported: Linux, MacOs, FreeBSD, OpenBsd, NetBSD, Windows
pub fn get_uptime(short: bool) -> String {
    match uptime_lib::get() {
        Ok(uptime) => {
            let uptime = uptime.num_seconds() as u64;
            let time = Duration::new(uptime, 0);
            let uptime = format_duration(time);
            let string = "Uptime: ".to_owned() + &uptime.to_string();
            if short {
                parse_for_shorthand_time(string).to_string()
            } else {
                string
            }
        }
        Err(err) => {
            eprintln!("Uptime: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn format_duration(duration: Duration) -> String {
    let sec = duration.as_secs();

    let days = sec / 86400;

    let sec = sec % 86400;

    let hours = sec / 3600;

    let sec = sec % 3600;

    let minutes = sec / 60;

    let seconds = sec % 60;

    let mut s = String::new();

    if days > 0 {
        s.write_fmt(format_args!("{} day", days)).unwrap();

        if days > 1 {
            s.push('s');
        }

        s.push_str(", ");
    }

    if hours > 0 || (days > 0) && (minutes > 0 || seconds > 0) {
        s.write_fmt(format_args!("{} hour", hours)).unwrap();

        if hours > 1 {
            s.push('s');
        }

        s.push_str(", ");
    }

    if minutes > 0 || (hours > 0 && seconds > 0) {
        s.write_fmt(format_args!("{} minute", minutes)).unwrap();

        if minutes > 1 {
            s.push('s');
        }

        s.push_str(", ");
    }

    if seconds > 0 {
        s.write_fmt(format_args!("{} second", seconds)).unwrap();

        if seconds > 1 {
            s.push('s');
        }

        s.push_str(", ");
    }

    debug_assert!(s.len() >= 2);

    if let Some(index) = s.as_str()[..(s.len() - 2)].rfind(", ") {
        s.insert_str(index + 2, "and ");
    }

    let len = s.len();

    let mut v = s.into_bytes();

    unsafe {
        v.set_len(len - 2);

        String::from_utf8_unchecked(v)
    }
}

fn parse_for_shorthand_time(uptime: String) -> String {
    let newtime = str::replace(&uptime, "years", "y");
    let newtime = str::replace(&newtime, "year", "y");
    let newtime = str::replace(&newtime, "days", "d");
    let newtime = str::replace(&newtime, "day", "d");
    let newtime = str::replace(&newtime, "hours", "h");
    let newtime = str::replace(&newtime, "hour", "h");
    let newtime = str::replace(&newtime, "minutes", "m");
    let newtime = str::replace(&newtime, "minute", "m");
    let newtime = str::replace(&newtime, "seconds", "s");
    str::replace(&newtime, "second", "s")
}
