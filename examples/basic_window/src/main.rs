use anyhow::Result;
use baldr::Application;
use baldr::widgets::{scaffold::Scaffold, appbar::AppBar, text::Text, Linux};

fn main() -> Result<()> {
    Text("Hello, world").widget();
    Ok(())
}
