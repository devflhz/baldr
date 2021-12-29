use anyhow::Result;
use baldr::widgets::button::Button;
use baldr::widgets::{appbar::AppBar, window::Window, text::Text};
use baldr::Application;
use baldr::widgets::window::WindowOptions;

fn main() -> Result<()> {
    let app = Application {
        app_id: "".to_string(),
        home: Window {
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
            options: WindowOptions {
                title: "My App".to_string(),
                default_height: 300,
                default_width: 400,
                ..Default::default()
            }
        },
    };
    app.create();
    Ok(())
}
