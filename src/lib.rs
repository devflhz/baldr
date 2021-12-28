use widgets::button::Button;

use crate::widgets::scaffold::Scaffold;
use crate::widgets::text::Text;
use crate::widgets::Widget;
use crate::widgets::appbar::AppBar;

pub mod platform;
pub mod widgets;

#[derive(Debug)]
pub struct Application<'a, T: Widget> {
    pub title: &'a str,
    pub app_id: &'a str,
    pub home: T,
}

impl<'a> Default for Application<'a, Scaffold<AppBar<Text<'a>>, Text<'a>>> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for Application<'a, Scaffold<AppBar<Text<'a>>, Text<'a>>> {

}

impl<'a> Default for Application<'a, Scaffold<AppBar<Text<'a>>, Button<'a>>> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for Application<'a, Scaffold<AppBar<Text<'a>>, Button<'a>>> {

}