use crate::widgets::Widget;
use crate::widgets::window::Window;

pub mod platform;
pub mod widgets;

pub trait AppCreation {
    fn create(self, bundle_id: &str);
}

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Clone)]
pub struct Application<'a> {
    pub window: Window<'a>,
}

#[cfg(target_os = "macos")]
#[derive(Debug)]
pub struct Application<'a> {
    pub window: Window<'a>,
}