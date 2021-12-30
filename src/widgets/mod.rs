use std::any::Any;
use std::fmt::Debug;
#[cfg(not(target_os = "macos"))]
use dyn_clone::{DynClone, clone_trait_object};

use self::text::Text;

pub mod appbar;
pub mod button;
pub mod container;
pub mod header_bar;
pub mod window;
pub mod text;

#[cfg(not(target_os = "macos"))]
pub trait Widget: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
}

#[cfg(target_os = "macos")]
pub trait Widget: Debug {
    fn as_any(&self) -> &dyn Any;
}

impl Default for Box<dyn Widget> {
    fn default() -> Self {
        Box::new(
            Text("")
        )
    }
}

#[cfg(not(target_os = "macos"))]
clone_trait_object!(Widget);

pub trait Native<T> {
    fn native(&self) -> T;
}
