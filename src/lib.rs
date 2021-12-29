use widgets::WidgetTrait;

use crate::widgets::text::Text;
use crate::widgets::Widget;

pub mod platform;
pub mod widgets;

#[derive(Debug)]
pub struct Application<'a> {
    pub title: &'a str,
    pub app_id: &'a str,
    pub home: Widget<'a>,
}

impl Application<'static> {}

impl WidgetTrait for Application<'static> {}
