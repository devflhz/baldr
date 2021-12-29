use cacao::text::Label;

use crate::widgets::{text::Text, DowncastWidget, WidgetTrait};

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Text("")
    }
}

impl<'a> WidgetTrait for Text<'a> {}

impl<'a> DowncastWidget<Label> for Text<'a> {
    fn downcast(&self) -> Label {
        let label = Label::new();
        label.set_text(self.0);
        label
    }
}
