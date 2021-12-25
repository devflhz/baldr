pub mod platform;
pub mod widgets;

pub trait AbstractApplication<T> {
    fn builder() -> Self;
    fn name(&mut self, name: &str) -> Self;
    fn application_id(&mut self, id: &str) -> Self;
    fn connect_activate(&self, window: Window<T>);
}

pub trait AbstractWindow<T> {
    fn builder() -> Self;
    fn title(self, title: &str) -> Self;
    fn default_width(self, w: i32) -> Self;
    fn default_height(self, h: i32) -> Self;
    fn set_width(self, w: i32) -> Self;
    fn set_height(self, h: i32) -> Self;
}

#[derive(Clone, Default)]
pub struct Window<T> {
    pub window: T,
    pub properties: WindowProperties
}

#[derive(Clone, Default)]
pub struct WindowProperties {
    default_width: i32,
    default_height: i32,
    width: i32,
    height: i32,
    title: String,
}

#[derive(Clone)]
pub struct Application {
    pub properties: AppProperties
}

#[derive(Clone, Default)]
pub struct AppProperties {
    name: String,
    app_id: String
}