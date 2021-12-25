use anyhow::Result;
#[cfg(target_os = "linux")]
use baldr::{AbstractApplication, AbstractWindow, Application, Window};
// #[cfg(target_os = "windows")]
use baldr::platform::win32::win32_window;

fn main() -> anyhow::Result<()> {
    let app = Application::builder()
        .application_id("com.edfloreshz.github");
    let window = Window::builder()
        .default_width(320)
        .default_height(320)
        .title("Example");
    app.connect_activate(window);
    win32_window()?;
    Ok(())
}
