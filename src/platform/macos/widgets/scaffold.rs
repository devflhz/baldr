use crate::widgets::appbar::AppBar;
use crate::widgets::scaffold::Scaffold;
use crate::widgets::text::Text;
use crate::widgets::WidgetTrait;

impl<'a> Default for Scaffold {
    fn default() -> Self {
        Self {
            app_bar: Box::new(AppBar {
                title: Default::default(),
            }),
            body: Box::new(Text("")),
        }
    }
}

impl<'a> WidgetTrait for Scaffold {}
