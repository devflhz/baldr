use crate::widgets::text::Text;

#[derive(Debug)]
pub struct Button<'a> {
    pub on_pressed: (),
    pub child: Text<'a>,
}
