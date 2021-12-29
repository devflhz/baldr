use crate::Widget;
use crate::widgets::appbar::AppBar;

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Clone, Default)]
pub struct Window {
    pub app_bar: Option<AppBar>,
    pub body: Box<dyn Widget>,
    pub title: String,
    pub default_width: i32,
    pub default_height: i32,
    pub width: i32,
    pub height: i32,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Default)]
pub struct Window {
    pub app_bar: Option<AppBar>,
    pub body: Box<dyn Widget>,
    pub title: String,
    pub default_width: i32,
    pub default_height: i32,
    pub width: i32,
    pub height: i32,
}