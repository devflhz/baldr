use std::any::Any;
use crate::widgets::{text::Text, Native};
use crate::Widget;
use gtk4::Label;

impl Widget for Text {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Native<Label> for Text {
    fn native(&self) -> Label {
        Label::new(Some(self.0.as_str()))
    }
}
