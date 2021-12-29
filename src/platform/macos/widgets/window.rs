use std::any::Any;
use cacao::macos::window::Window as NSWindow;
use crate::widgets::Native;
use crate::Widget;
use crate::widgets::window::Window;

impl Widget for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Native<NSWindow> for Window {
    fn native(&self) -> NSWindow {
        NSWindow::default()
    }
}
