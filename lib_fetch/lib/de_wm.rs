use std::env;

#[derive(Debug, PartialEq, Eq)]
pub enum DesktopEnvironment {
    Unknown,
    Cinnamon,
    Enlightenment,
    GNOME,
    KDE,
    LXDE,
    LXQT,
    MacOs,
    Mate,
    Unity,
    Windows,
    XFCE,
}

pub fn detect() -> DesktopEnvironment {
    if cfg!(target_os = "macos") {
        DesktopEnvironment::MacOs
    } else if cfg!(target_os = "windows") {
        DesktopEnvironment::Windows
    } else if cfg!(target_family = "unix") {
        match env::var("XDG_CURRENT_DESKTOP")
            .expect("Failed to parse environment variables")
            .as_str()
        {
            "Cinnamon" => DesktopEnvironment::Cinnamon,
            "ENLIGHTENMENT" => DesktopEnvironment::Enlightenment,
            "GNOME" => DesktopEnvironment::GNOME,
            "KDE" => DesktopEnvironment::KDE,
            "LXDE" => DesktopEnvironment::LXDE,
            "LXQt" => DesktopEnvironment::LXQT,
            "MATE" => DesktopEnvironment::Mate,
            "Unity" => DesktopEnvironment::Unity,
            "X-Cinnamon" => DesktopEnvironment::Cinnamon,
            "XFCE" => DesktopEnvironment::XFCE,
            _ => DesktopEnvironment::Unknown,
        }
    } else {
        DesktopEnvironment::Unknown
    }
}

pub fn get_de() -> DesktopEnvironment {
    let de: DesktopEnvironment = detect();
    de
}
