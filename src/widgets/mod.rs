pub mod button;
pub mod scaffold;
pub mod appbar;
pub mod text;

pub trait Widget<W, L, M>: Windows<W> + Linux<L> + Mac<M>  {}

pub trait Windows<W> {
    fn widget(&self) -> W;
}

pub trait Linux<L> {
    fn widget(&self) -> L;
}

pub trait Mac<M> {
    fn widget(&self) -> M;
}