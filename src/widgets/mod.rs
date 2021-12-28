pub mod button;
pub mod scaffold;
pub mod appbar;
pub mod text;
pub mod header_bar;
pub mod container;

pub trait Widget: Default + Sized {}

pub trait DowncastWidget<T> {
    fn downcast(&self) -> T;
}