use crate::widgets::Widget;
use crate::widgets::window::Window;

pub mod platform;
pub mod widgets;

pub trait AppCreation {
    fn create(self, bundle_id: &str);
}

#[derive(Debug, Clone)]
pub struct Application<'a> {
    pub window: Window<'a>,
}