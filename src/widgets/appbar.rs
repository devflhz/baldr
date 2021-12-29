use super::WidgetTrait;

#[derive(Debug)]
pub struct AppBar<T: WidgetTrait> {
    pub title: T,
}
