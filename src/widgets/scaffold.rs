use crate::Widget;

#[derive(Debug)]
pub struct Scaffold<A: Widget, B: Widget> {
    pub app_bar: A,
    pub body: B
}