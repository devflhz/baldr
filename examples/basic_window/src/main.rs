use anyhow::Result;
use baldr::widgets::button::Button;
use baldr::widgets::{appbar::AppBar, scaffold::Scaffold, text::Text};
use baldr::Application;

fn main() -> Result<()> {
    let app = Application {
        title: "",
        app_id: "",
        home: &Scaffold {
            app_bar: &AppBar {
                title: Text("My First App"),
            },
            body: &Button {
                child: Text("Press"),
                ..Default::default()
            },
        },
    };
    println!("{:#?}", app);
    Ok(())
}
