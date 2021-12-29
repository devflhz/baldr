use std::any::Any;
use std::fmt::Debug;
use dyn_clone::DynClone;
use dyn_clone::clone_trait_object;

pub mod appbar;
pub mod button;
pub mod container;
pub mod header_bar;
pub mod window;
pub mod text;

pub trait Widget: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
}

clone_trait_object!(Widget);

pub trait Native<T> {
    fn native(&self) -> T;
}
