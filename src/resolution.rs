use shells::*;
#[cfg(feature = "libx11")]
pub fn get_resolution(refresh: bool) -> String {
    use std::ptr;
    use x11::xlib::{XCloseDisplay, XOpenDisplay, XScreenCount, XScreenOfDisplay};
    let mut v: Vec<libc::c_int> = Vec::with_capacity(2);
    unsafe {
        // open a display
        let display = XOpenDisplay(ptr::null());
        // return the number of available screens
        let count_screens = XScreenCount(display);

        println!("Number of screens: {}", count_screens);
        for i in 0..count_screens {
            let screen = *XScreenOfDisplay(display, i);
            v.push(screen.width);
            v.push(screen.height);
            //println!("Resolution: {}x{}", screen.width, screen.height);
        }
        // close the display
        XCloseDisplay(display);
    }
    if v.len() > 1 && !refresh {
        v[0].to_string() + "x" + &v[1].to_string()
    } else if v.len() > 1 && refresh {
        v[0].to_string() + "x" + &v[1].to_string() + "60Hz"
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(feature = "libx11"))]
pub fn get_resolution(refresh: bool) -> String {
    use crate::packages::usable;
    if !refresh {
        if usable("/usr/bin/xwininfo") {
            xwininfo(refresh)
        } else if usable("/usr/bin/xrandr") {
            xrandr(refresh)
        } else if usable("/usr/bin/xdpyinfo") {
            xdpyinfo(refresh)
        } else {
            "Unknown".to_string()
        }
    } else {
        if usable("/usr/bin/xrandr") {
            xrandr(refresh)
        } else if usable("/usr/bin/xwininfo") {
            xwininfo(refresh)
        } else if usable("/usr/bin/xdpyinfo") {
            xdpyinfo(refresh)
        } else {
            "Unknown".to_string()
        }
    }
}

pub fn xrandr(refresh: bool) -> String {
    let v: Vec<&str> = Vec::with_capacity(7);
    let (_code, stdout, _stderr) = sh!("xrandr | grep '*'");

    let v: Vec<&str> = stdout[..].split_whitespace().collect();
    if !refresh {
        v[0].to_string()
    } else {
        v[0].to_string() + " " + v[1].split('.').take(1).next().unwrap_or("") + "Hz"
    }
}

pub fn xwininfo(refresh: bool) -> String {
    let (_code, stdout, _stderr) = sh!("xwininfo -root | awk -F':' '/Width|Height/ {{printf $2}}'");

    str::replace(&stdout[..].trim(), " ", "x").to_string()
}

pub fn xdpyinfo(refresh: bool) -> String {
    let (_code, stdout, _stderr) = sh!("xdpyinfo | awk '/dimensions/{{printf $2}}'");
    stdout[..].to_string()
}
