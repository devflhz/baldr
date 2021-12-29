use crate::widgets::{text::Text, DowncastWidget};
use crate::WidgetTrait;
use gtk4::Label;

impl<'a> WidgetTrait for Text<'a> {}

impl<'a> DowncastWidget<Label> for Text<'a> {
    fn downcast(&self) -> Label {
        Label::new(Some(self.0))
    }
}
