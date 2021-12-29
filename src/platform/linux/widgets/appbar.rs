use crate::widgets::appbar::AppBar;
use crate::widgets::DowncastWidget;
use crate::{Text, Widget};
use gtk4::{HeaderBar, Label};

impl<'a> Default for AppBar<Text<'a>> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for AppBar<Text<'a>> {}

impl<'a> DowncastWidget<HeaderBar> for AppBar<Text<'a>> {
    fn downcast(&self) -> HeaderBar {
        let header = HeaderBar::new();
        header.set_title_widget(Some(&Label::new(Some(self.title.0))));
        header
    }
}
