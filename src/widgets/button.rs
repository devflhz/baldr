use crate::widgets::text::Text;

#[derive(Debug, Default, Clone)]
pub struct Button {
    pub on_pressed: (),
    pub child: Text,
}
