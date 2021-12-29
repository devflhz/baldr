use crate::widgets::window::Window;
use crate::widgets::Widget;

impl Widget for Window {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
