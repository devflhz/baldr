use anyhow::Result;
use baldr::platform::macos::appkit::Application;
use baldr::widgets::button::Button;
use baldr::widgets::{appbar::AppBar, window::Window, text::Text};
use baldr::{
    AppCreation
};

fn main() -> Result<()> {
    let app = Application {
        props: Window {
            title: "My App".to_string(),
            default_height: 300,
            default_width: 400,
            app_bar: Some(
                AppBar {
                    title: Text("My First App".to_string())
                }
            ),
            body: Box::new(
                Button {
                    on_pressed: {
                        println!("Pressed!")
                    },
                    child: Text("Press me!".to_string())
                }
            ),
            ..Default::default()
        },
        ..Default::default()
    };
    app.create("com.github.edfloreshz");
    Ok(())
}
