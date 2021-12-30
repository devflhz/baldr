use crate::widgets::button::Button;
use crate::widgets::{Native, Widget};
use cacao::button::Button as AKButton;

impl Widget for Button<'static> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<'a> Native<AKButton> for Button<'a> {
    fn native(&self) -> AKButton {
        println!("{}", self.child.0);
        let mut button = AKButton::new(self.child.0);
        let on_pressed = self.clone().on_pressed;
        button.set_action(move || on_pressed);
        button
    }
}
