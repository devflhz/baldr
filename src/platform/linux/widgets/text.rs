use std::any::Any;
use crate::widgets::{text::Text, Native};
use crate::Widget;
use gtk4::Label;

impl Widget for Text<'static> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> Native<Label> for Text<'a> {
    fn native(&self) -> Label {
        Label::new(Some(self.0))
    }
}
