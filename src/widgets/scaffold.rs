use crate::Widget;

#[derive(Debug)]
pub struct Scaffold<'a> {
    pub app_bar: Widget<'a>,
    pub body: Widget<'a>,
}
