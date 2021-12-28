use crate::Widget;

#[derive(Debug)]
pub struct AppBar<T: Widget> {
    pub title: T
}
