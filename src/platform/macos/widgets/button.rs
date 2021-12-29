use crate::widgets::button::Button;
use crate::widgets::{DowncastWidget, WidgetTrait};
use cacao::button::Button as AKButton;

impl<'a> WidgetTrait for Button<'a> {}

impl<'a> DowncastWidget<AKButton> for Button<'a> {
    fn downcast(&self) -> AKButton {
        let button = AKButton::new(self.child.0);
        button
    }
}
