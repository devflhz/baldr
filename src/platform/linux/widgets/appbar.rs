use std::any::Any;
use crate::widgets::appbar::AppBar;
use crate::widgets::Native;
use crate::Widget;
use gtk4::{HeaderBar, Label};

impl<'a> Widget for AppBar {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> Native<HeaderBar> for AppBar {
    fn native(&self) -> HeaderBar {
        let header = HeaderBar::new();
        header.set_title_widget(Some(&Label::new(Some(self.title.0.as_str()))));
        header
    }
}
