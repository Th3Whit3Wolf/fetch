// Platforms supported: Linux, MacOs, FreeBSD, OpenBsd, NetBSD, Windows
#[allow(clippy::cyclomatic_complexity)]
pub fn get_uptime(short: bool) -> String {
    match uptime_lib::get() {
        Ok(uptime) => {
            let secs: u64 = uptime.num_seconds() as u64;
            let uptime: String = "Uptime: ".to_string();

            const YEAR: u64 = 31_536_000;
            const DAY: u64 = 86_400;
            const HOUR: u64 = 3_600;
            const MINUTE: u64 = 60;

            let years: u64 = secs / YEAR;
            let days: u64 = (secs - (years * YEAR)) / DAY;
            let hours: u64 = (secs - (years * YEAR) - (days * DAY)) / HOUR;
            let minutes: u64 = (secs - (years * YEAR) - (days * DAY) - (hours * HOUR)) / MINUTE;
            let seconds: u64 =
                secs - (years * YEAR) - (days * DAY) - (hours * HOUR) - (minutes * MINUTE);

            match years as u64 {
                0 => match days as u64 {
                    0 => match hours as u64 {
                        0 => match minutes {
                            0 => match seconds {
                                0 => {
                                    let uptime = uptime + "0seca";
                                    if short {
                                        parse_for_shorthand_time(uptime)
                                    } else {
                                        uptime
                                    }
                                }
                                1 => {
                                    let uptime = uptime + "1sec";
                                    if short {
                                        parse_for_shorthand_time(uptime)
                                    } else {
                                        uptime
                                    }
                                }

                                2...59 => {
                                    let uptime = uptime + &seconds.to_string() + "secs";
                                    if short {
                                        parse_for_shorthand_time(uptime)
                                    } else {
                                        uptime
                                    }
                                }
                                _ => unreachable!(),
                            },
                            1 => {
                                let uptime = uptime + "1min";
                                match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            2...59 => {
                                let uptime = uptime + &minutes.to_string() + "mins";
                                match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        },
                        1 => {
                            let uptime = uptime + "1hour";
                            match minutes {
                                0 => match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1min";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...59 => {
                                    let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        2...23 => {
                            let uptime = uptime + &hours.to_string() + "hours";
                            match minutes {
                                0 => match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1min";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...59 => {
                                    let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    },
                    1 => {
                        let uptime = uptime + "1day";
                        match hours {
                            0 => match minutes {
                                0 => match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1min";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            if short {
                                                let uptime = uptime + ", 1sec";
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...59 => {
                                    let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            },
                            1 => {
                                let uptime = uptime + ", 1hour";
                                match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            2...23 => {
                                let uptime = uptime + &hours.to_string() + ", hours";
                                match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    2...364 => {
                        let uptime = uptime + &days.to_string() + "days";
                        match hours {
                            0 => match minutes {
                                0 => match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1min";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...59 => {
                                    let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            },
                            1 => {
                                let uptime = uptime + "1hour, ";
                                match minutes {
                                    0 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            2...23 => {
                                let uptime = uptime + &hours.to_string() + "hours";
                                match minutes {
                                    0 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                },
                1 => {
                    let uptime = uptime + "1year";
                    match days as u64 {
                        0 => match hours as u64 {
                            0 => match minutes {
                                0 => match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1min";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...59 => {
                                    let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            },
                            1 => {
                                let uptime = uptime + ", 1hour";
                                match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => uptime + ", " + &seconds.to_string() + "secs",
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            2...23 => {
                                let uptime = uptime + ", " + &hours.to_string() + "hours";
                                match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        },
                        1 => {
                            let uptime = uptime + ", 1day";
                            match hours {
                                0 => match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1hour";
                                    match minutes {
                                        0 => match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        },
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...23 => {
                                    let uptime = uptime + &hours.to_string() + ", hours";
                                    match minutes {
                                        0 => match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        },
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        2...364 => {
                            let uptime = uptime + &days.to_string() + ", days";
                            match hours {
                                0 => match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1hour, ";
                                    match minutes {
                                        0 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...23 => {
                                    let uptime = uptime + ", " + &hours.to_string() + "hours, ";
                                    match minutes {
                                        0 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                2...584_942_417_355 => {
                    let uptime = uptime + &years.to_string() + "years";
                    match days as u64 {
                        0 => match hours as u64 {
                            0 => match minutes {
                                0 => match seconds {
                                    0 => {
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    1 => {
                                        let uptime = uptime + ", 1sec";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &seconds.to_string() + "secs";
                                        if short {
                                            parse_for_shorthand_time(uptime)
                                        } else {
                                            uptime
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1min";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...59 => {
                                    let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                    match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }

                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            },
                            1 => {
                                let uptime = uptime + ", 1hour";
                                match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            2...23 => {
                                let uptime = uptime + "," + &hours.to_string() + "hours";
                                match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        },
                        1 => {
                            let uptime = uptime + ", 1day";
                            match hours {
                                0 => match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => uptime + ", " + &seconds.to_string() + "secs",
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + ", 1hour";
                                    match minutes {
                                        0 => match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        },
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...23 => {
                                    let uptime = uptime + ", " + &hours.to_string() + "hours";
                                    match minutes {
                                        0 => match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        },
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        2...364 => {
                            let uptime = uptime + " ," + &days.to_string() + "days";
                            match hours {
                                0 => match minutes {
                                    0 => match seconds {
                                        0 => {
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1sec";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &seconds.to_string() + "secs";
                                            if short {
                                                parse_for_shorthand_time(uptime)
                                            } else {
                                                uptime
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    1 => {
                                        let uptime = uptime + ", 1min";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    2...59 => {
                                        let uptime = uptime + ", " + &minutes.to_string() + "mins";
                                        match seconds {
                                            0 => {
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            1 => {
                                                let uptime = uptime + ", 1sec";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            2...59 => {
                                                let uptime =
                                                    uptime + ", " + &seconds.to_string() + "secs";
                                                if short {
                                                    parse_for_shorthand_time(uptime)
                                                } else {
                                                    uptime
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => unreachable!(),
                                },
                                1 => {
                                    let uptime = uptime + "1hour, ";
                                    match minutes {
                                        0 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                2...23 => {
                                    let uptime = uptime + ", " + &hours.to_string() + "hours";
                                    match minutes {
                                        0 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        1 => {
                                            let uptime = uptime + ", 1min";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        2...59 => {
                                            let uptime =
                                                uptime + ", " + &minutes.to_string() + "mins";
                                            match seconds {
                                                0 => {
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                1 => {
                                                    let uptime = uptime + ", 1sec";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                2...59 => {
                                                    let uptime = uptime
                                                        + ", "
                                                        + &seconds.to_string()
                                                        + "secs";
                                                    if short {
                                                        parse_for_shorthand_time(uptime)
                                                    } else {
                                                        uptime
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        Err(err) => err,
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
