use objc2::{AllocAnyThread, MainThreadMarker};
use objc2_app_kit::{NSApplication, NSImage};
use objc2_foundation::NSData;

pub fn set_application_icon() {
    use lunaria_assets::icon::APPLICATION_ICON;

    let Some(main_thread) = MainThreadMarker::new() else {
        eprintln!("Failed to set application icon: not running on the main thread");
        return;
    };

    let data = NSData::with_bytes(APPLICATION_ICON);
    let Some(icon) = NSImage::initWithData(NSImage::alloc(), &data) else {
        eprintln!("Failed to set application icon: invalid icon data");
        return;
    };

    let app = NSApplication::sharedApplication(main_thread);

    // SAFETY: We pass a valid, non-null NSImage
    unsafe { app.setApplicationIconImage(Some(&icon)) };
}
