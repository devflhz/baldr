use std::any::Any;
use crate::widgets::window::Window;

use crate::widgets::text::Text;
use crate::widgets::Widget;

pub mod platform;
pub mod widgets;

#[derive(Debug, Clone)]
pub struct Application {
    pub app_id: String,
    pub home: Window,
}

impl Widget for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
