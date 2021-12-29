use crate::widgets::button::Button;
use crate::widgets::text::Text;
use crate::widgets::{DowncastWidget, WidgetTrait};
use cacao::button::Button as AKButton;

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Button {
            on_pressed: (),
            child: Text::default(),
        }
    }
}

impl<'a> WidgetTrait for Button<'a> {}

impl<'a> DowncastWidget<AKButton> for Button<'a> {
    fn downcast(&self) -> AKButton {
        let button = AKButton::new(self.child.0);
        button
    }
}
