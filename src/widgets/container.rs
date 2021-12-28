use crate::Widget;

pub struct Container<T: Widget> {
    _children: T
}