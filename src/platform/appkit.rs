#[cfg(target_os = "macos")]
struct NSWindow {
    title: String,
    default_width: u32,
    default_height: u32
}