use anyhow::Result;
use baldr::{AbstractApplication, AbstractWindow, Application, Window};

fn main() -> Result<()> {
    let app = Application::builder()
        .application_id("com.edfloreshz.github");
    let window = Window::builder()
        .default_width(320)
        .default_height(620)
        .title("Example");
    app.connect_activate(window);
    Ok(())
}
