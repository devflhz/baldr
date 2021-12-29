use std::any::Any;
use crate::widgets::window::Window;
use crate::Widget;
use crate::Text;

impl Default for Window {
    fn default() -> Self {
        Self {
            app_bar: Default::default(),
            options: Default::default(),
            body: Box::new(Text("".to_string()))
        }
    }
}

impl Widget for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
