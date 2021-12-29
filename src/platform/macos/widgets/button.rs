use crate::widgets::button::Button;
use crate::widgets::{Native, Widget};
use cacao::button::Button as AKButton;

impl Widget for Button {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Native<AKButton> for Button {
    fn native(&self) -> AKButton {
        let button = AKButton::new(self.child.0.as_str());
        button
    }
}
