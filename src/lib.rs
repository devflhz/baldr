pub mod platform;

pub trait AbstractApplication<T> {
    fn builder() -> Self;
    fn name(&mut self, name: &str) -> Self;
    fn application_id(&mut self, id: &str) -> Self;
    fn connect_activate(&self, window: Window<T>);
}

pub trait AbstractWindow<T> {
    fn builder() -> Self;
    fn title(&mut self, title: &str) -> Self;
    fn default_width(&mut self, w: i32) -> Self;
    fn default_height(&mut self, h: i32) -> Self;
    fn set_width(&mut self, w: i32) -> Self;
    fn set_height(&mut self, h: i32) -> Self;
    fn build(&self) -> T;
}

#[derive(Clone)]
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
pub struct Application<T> {
    pub application: T,
    pub properties: AppProperties
}

#[derive(Clone, Default)]
pub struct AppProperties {
    name: String,
    app_id: String
}