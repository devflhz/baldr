use crate::Widget;
use crate::widgets::appbar::AppBar;

#[derive(Debug, Clone)]
pub struct Window {
    pub app_bar: Option<AppBar>,
    pub body: Box<dyn Widget>,
    pub options: WindowOptions,
}

#[derive(Debug, Clone, Default)]
pub struct WindowOptions {
    pub title: String,
    pub default_width: i32,
    pub default_height: i32,
    pub width: i32,
    pub height: i32,
}
