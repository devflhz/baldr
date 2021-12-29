use crate::widgets::button::Button;
use crate::widgets::{Native, Widget};
use cacao::button::Button as AKButton;

impl<'a> Widget for Button<'a> {}

impl<'a> Native<AKButton> for Button<'a> {
    fn native(&self) -> AKButton {
        let button = AKButton::new(self.child.0);
        button
    }
}
