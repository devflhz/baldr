use crate::widgets::appbar::AppBar;
use crate::widgets::DowncastWidget;
use crate::WidgetTrait;
use gtk4::{HeaderBar, Label};

impl<'a> WidgetTrait for AppBar<'a> {}

impl<'a> DowncastWidget<HeaderBar> for AppBar<'a> {
    fn downcast(&self) -> HeaderBar {
        let header = HeaderBar::new();
        header.set_title_widget(Some(&Label::new(Some(self.title.0))));
        header
    }
}
