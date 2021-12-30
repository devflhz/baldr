use crate::widgets::text::Text;

#[derive(Debug, Default, Clone)]
pub struct Button<'a> {
    pub on_pressed: (),
    pub child: Text<'a>,
}

impl<'a> Button<'a> {
    pub fn new(title: &'a str, on_pressed: ()) -> Box<Self> {
        Box::new(
            Self {
                on_pressed,
                child: Text(title),
            }
        )
    }
}