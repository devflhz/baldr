use crate::widgets::{text::Text, DowncastWidget, Widget};
use gtk4::Label;

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for Text<'a> {}

impl<'a> DowncastWidget<Label> for Text<'a> {
    fn downcast(&self) -> Label {
        Label::new(Some(self.0))
    }
}
