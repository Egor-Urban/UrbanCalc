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