use std::fmt::Debug;

pub mod appbar;
pub mod button;
pub mod container;
pub mod header_bar;
pub mod scaffold;
pub mod text;

pub trait WidgetTrait: Debug {}

pub type Widget<'a> = &'a dyn WidgetTrait;

pub trait DowncastWidget<T> {
    fn downcast(&self) -> T;
}
