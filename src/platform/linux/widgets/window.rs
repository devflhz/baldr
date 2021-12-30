use std::any::Any;
use crate::widgets::window::Window;
use crate::Widget;

impl Widget for Window<'static> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
