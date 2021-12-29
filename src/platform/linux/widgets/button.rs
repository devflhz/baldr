use crate::widgets::button::Button;
use crate::widgets::DowncastWidget;
use crate::WidgetTrait;
use gtk4::Button as GTKButton;

impl<'a> WidgetTrait for Button<'a> {}

impl<'a> DowncastWidget<GTKButton> for Button<'a> {
    fn downcast(&self) -> GTKButton {
        GTKButton::builder().label((*self).child.0).build()
    }
}
