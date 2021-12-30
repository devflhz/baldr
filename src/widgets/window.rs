#[cfg(target_os = "macos")]
use cacao::view::View;

use crate::Widget;
use crate::widgets::appbar::AppBar;

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Clone, Default)]
pub struct Window<'a> {
    pub app_bar: Option<AppBar<'a>>,
    pub body: Box<dyn Widget>,
    pub title: &'a str,
    pub default_width: i32,
    pub default_height: i32,
    pub width: i32,
    pub height: i32,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Default)]
pub struct Window<'a> {
    pub app_bar: Option<AppBar<'a>>,
    pub body: Box<dyn Widget>,
    pub title: &'a str,
    pub default_width: i32,
    pub default_height: i32,
    pub width: i32,
    pub height: i32,
    pub view: View
}