use crate::widgets::Widget;

pub mod platform;
pub mod widgets;

pub trait AppCreation {
    fn create(self, bundle_id: &str);
}