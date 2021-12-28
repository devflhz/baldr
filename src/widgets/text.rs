use gtk4::Label;
use crate::widgets::{Linux, Mac, Widget, Windows};

#[derive(Default)]
pub struct Text<'a>(pub &'a str);

impl<'a> Windows<String> for Text<'a> {
    fn widget(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> Linux<Label> for Text<'a> {
    fn widget(&self) -> Label {
        Label::new(Some(self.0))
    }
}

impl<'a> Mac<String> for Text<'a> {
    fn widget(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> Widget<String, Label, String> for Text<'a> {}