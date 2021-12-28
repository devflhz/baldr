pub mod platform;
pub mod widgets;

pub struct Application<'a, T> {
    pub title: &'a str,
    pub app_id: &'a str,
    pub home: T,
}
