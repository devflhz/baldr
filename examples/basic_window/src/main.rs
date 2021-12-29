use anyhow::Result;
use baldr::widgets::button::Button;
use baldr::widgets::{appbar::AppBar, scaffold::Scaffold, text::Text};
use baldr::Application;

fn main() -> Result<()> {
    let app = Application {
        title: "",
        app_id: "",
        home: Box::new(Scaffold {
            app_bar: Box::new(AppBar {
                title: Text("My First App"),
            }),
            body: Box::new(Button {
                child: Text("Press"),
                on_pressed: { println!("Pressed!") },
            }),
        }),
    };
    println!("{:#?}", app);
    Ok(())
}
