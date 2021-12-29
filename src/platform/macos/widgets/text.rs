use cacao::text::Label;

use crate::widgets::{text::Text, Native, Widget};

impl Widget for Text {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Native<Label> for Text {
    fn native(&self) -> Label {
        let label = Label::new();
        label.set_text(self.0.as_str());
        label
    }
}
