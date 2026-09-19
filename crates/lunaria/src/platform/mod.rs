#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use macos::set_application_icon;

#[cfg(target_os = "windows")]
pub use windows::set_application_icon;
