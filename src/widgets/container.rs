use super::WidgetTrait;

pub struct Container<T: WidgetTrait> {
    _children: T,
}
