use crate::widgets::text::Text;

pub struct Button<'a> {
    pub on_pressed: (),
    pub child: Text<'a>
}