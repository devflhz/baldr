use cacao::text::Label;

use crate::widgets::{Widget, text::Text, DowncastWidget};

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Text("")
    }
}

impl<'a> Widget for Text<'a> {}

impl<'a> DowncastWidget<Label> for Text<'a> {
    fn downcast(&self) -> Label {
        let label = Label::new();
        label.set_text(self.0);
        label
    }
}