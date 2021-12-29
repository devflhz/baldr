use crate::widgets::appbar::AppBar;
use crate::widgets::scaffold::Scaffold;
use crate::widgets::text::Text;
use crate::Widget;

impl<'a> Default for Scaffold<AppBar<Text<'a>>, Text<'a>> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for Scaffold<AppBar<Text<'a>>, Text<'a>> {}
