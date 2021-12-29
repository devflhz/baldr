use crate::widgets::button::Button;
use crate::widgets::DowncastWidget;
use crate::Widget;
use gtk4::Button as GTKButton;

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for Button<'a> {}

impl<'a> DowncastWidget<GTKButton> for Button<'a> {
    fn downcast(&self) -> GTKButton {
        let button = GTKButton::builder().label(self.clone().child.0).build();
        // button.connect_clicked(|button| self.clone().on_pressed);
        button
    }
}
