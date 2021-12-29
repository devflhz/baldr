use super::WidgetTrait;

#[derive(Debug, Default)]
pub struct AppBar<T: WidgetTrait> {
    pub title: T,
}
