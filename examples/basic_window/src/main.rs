use anyhow::Result;
use baldr::widgets::button::Button;
use baldr::widgets::{appbar::AppBar, window::Window};
use baldr::{AppCreation, Application};

fn main() -> Result<()> {
    let app = Application {
        window: Window {
            title: "My App",
            default_height: 300,
            default_width: 400,
            app_bar: AppBar::new("My First App"),
            body: Button::new("Press me!", println!("Pressed!")),
            ..Default::default()
        },
    };
    app.create("com.github.edfloreshz");
    Ok(())
}
