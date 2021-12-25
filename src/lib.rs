pub mod platform;

pub trait AbstractWindow<T> {
    fn builder() -> Self;
    fn title(&mut self, title: &str) -> Self;
    fn default_width(&mut self, w: i32) -> Self;
    fn default_height(&mut self, h: i32) -> Self;
    fn build(&self) -> T;
}

pub trait AbstractApplication<T> {
    fn builder() -> Self;
    fn application_id(&mut self, id: &str) -> Self;
    fn connect_activate(&self, window: Window<T>);
}

#[derive(Clone)]
pub struct Window<T> {
    pub window: T
}

#[derive(Clone)]
pub struct Application<T> {
    pub application: T
}