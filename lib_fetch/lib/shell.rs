use std::env::var_os;

pub enum Shell {
    Windows,
    Bash,
    Tcsh,
    Zsh,
    Ksh,
    Unknown,
}

pub fn shell() -> Shell {
    if cfg!(windows) {
        Shell::Windows
    } else {
        if let Some(shell) = var_os("BASH") {
            if shell.to_string_lossy().ends_with("/bash") {
                return Shell::Bash;
            }
        }
        if let Some(zsh) = var_os("ZSH_NAME") {
            if zsh.to_string_lossy() == "zsh" {
                return Shell::Zsh;
            }
        }
        if let Some(shell) = var_os("shell") {
            if shell.to_string_lossy().ends_with("/tcsh") {
                return Shell::Tcsh;
            }
        }
        match var_os("SHELL") {
            None => Shell::Bash,
            Some(oss) => {
                if oss.to_string_lossy().ends_with("/bash") {
                    Shell::Bash
                } else if oss.to_string_lossy().ends_with("/ksh") {
                    Shell::Ksh
                } else if oss.to_string_lossy().ends_with("/zsh") {
                    Shell::Zsh
                } else if oss.to_string_lossy().ends_with("/tcsh") {
                    Shell::Tcsh
                } else {
                    Shell::Bash
                } // many shells support export foo=bar
            }
        }
    }
}

/*
    Todo
    ADD more shells
    ADD Get Shell Versions(change get_name to run cmd for version &parse)
*/

impl Shell {
    pub fn get_name(&self) -> &'static str {
        match *self {
            Shell::Windows => "Windows",
            Shell::Bash => "Bash",
            Shell::Tcsh => "Tcsh",
            Shell::Zsh => "Zsh",
            Shell::Ksh => "Ksh",
            Shell::Unknown => "Unknown",
        }
    }
}
