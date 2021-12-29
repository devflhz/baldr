use std::any::Any;
use crate::widgets::window::Window;
use crate::Widget;
use crate::Text;

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            default_width: 0,
            default_height: 0,
            width: 0,
            height: 0,
            app_bar: Default::default(),
            body: Box::new(Text("".to_string())),
        }
    }
}

impl Widget for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
