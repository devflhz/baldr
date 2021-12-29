use cacao::text::Label;

use crate::widgets::{text::Text, Native, Widget};

impl<'a> Widget for Text<'a> {}

impl<'a> Native<Label> for Text<'a> {
    fn native(&self) -> Label {
        let label = Label::new();
        label.set_text(self.0);
        label
    }
}
