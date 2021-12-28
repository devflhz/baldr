use cacao::button::Button as AKButton;
use crate::Widget;
use crate::widgets::button::Button;
use crate::widgets::DowncastWidget;
use crate::widgets::text::Text;

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Button {
            on_pressed: (),
            child: Text::default(),
        }
    }
}

impl<'a> Widget for Button<'a> {

}

impl<'a> DowncastWidget<AKButton> for Button<'a> {
    fn downcast(&self) -> AKButton {
        let button = AKButton::new(self.child.0);
        button
    }
}