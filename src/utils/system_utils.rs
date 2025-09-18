use std::process::Command;



pub fn get_os() -> &'static str {
    #[cfg(target_os = "windows")]
    { "Windows" }

    #[cfg(target_os = "macos")]
    { "macOS" }

    #[cfg(target_os = "linux")]
    { "Linux" }

    #[cfg(target_os = "android")]
    { "Android" }

    #[cfg(target_os = "ios")]
    { "iOS" }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "android",
        target_os = "ios"
    )))]
    { "Unknown" }
}



pub fn get_theme(os: &str) -> &'static str {
    match os {
        // NOT TESTED
        "Windows" => {
            #[cfg(target_os = "windows")]
            {
                use winreg::enums::*;
                use winreg::RegKey;

                let hkey = RegKey::predef(HKEY_CURRENT_USER);
                if let Ok(key) = hkey.open_subkey(
                    "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"
                ) {
                    match key.get_value::<u32, _>("AppsUseLightTheme") {
                        Ok(val) => {
                            if val == 0 { return "dark"; } else { return "light"; }
                        }
                        Err(_) => return "Unknown",
                    }
                }
            }
            "Unknown"
        }

        // NOT TESTED
        "macOS" => {
            #[cfg(target_os = "macos")]
            {
                if let Ok(output) = Command::new("defaults")
                    .args(["read", "-g", "AppleInterfaceStyle"])
                    .output()
                {
                    let style = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if style.eq_ignore_ascii_case("Dark") {
                        return "dark";
                    } else if style.is_empty() {
                        return "light";
                    }
                }
            }
            "Unknown"
        }

        // Only gnome
        "Linux" => {
            #[cfg(target_os = "linux")]
            {
                if let Ok(output) = Command::new("gsettings")
                    .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
                    .output()
                {
                    let theme = String::from_utf8_lossy(&output.stdout).to_lowercase();
                    if theme.contains("dark") {
                        return "dark";
                    } else if !theme.is_empty() {
                        return "light";
                    }
                }
            }
            "Unknown"
        }
        _ => "Unknown",
    }
}