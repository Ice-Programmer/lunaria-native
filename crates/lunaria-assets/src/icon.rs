#[cfg(target_os = "macos")]
pub const APPLICATION_ICON: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/app/icon.icns"));

#[cfg(target_os = "windows")]
pub const APPLICATION_ICON: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/app/icon.ico"));

#[cfg(target_os = "linux")]
pub const APPLICATION_ICON: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/app/icon.png"));
