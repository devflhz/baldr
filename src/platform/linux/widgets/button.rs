use std::any::Any;
use crate::widgets::button::Button;
use crate::widgets::Native;
use crate::Widget;
use gtk4::prelude::ButtonExt;
use gtk4::Button as GTKButton;

impl Widget for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Native<GTKButton> for Button {
    fn native(&self) -> GTKButton {
        let button = GTKButton::builder().label((*self).child.0.as_str()).build();
        let btn = self.clone();
        button.connect_activate(move |_| btn.on_pressed);
        button
    }
}
