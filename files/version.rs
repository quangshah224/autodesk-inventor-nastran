//! Static build info for Autodesk Inventor Nastran.

pub const BUILD_VERSION: &str  = "v1.4.12";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "30b8a20e7144";
pub const BUILD_TIMESTAMP: u64 = 1703064057;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Autodesk Inventor Nastran", BUILD_VERSION, BUILD_CHANNEL)
}
