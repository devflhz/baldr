use anyhow::Result;
use baldr::Application;
use baldr::widgets::{
    scaffold::Scaffold,
    appbar::AppBar,
    text::Text
};

fn main() -> Result<()> {
    let app = Application {
        title: "",
        app_id: "",
        home: Scaffold {
            app_bar: AppBar {
                title: Text("My First App")
            },
            body: Text("Hello World.")
        },
    };
    println!("{:#?}", app);
    Ok(())
}
