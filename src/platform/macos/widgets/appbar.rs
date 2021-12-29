use crate::widgets::appbar::AppBar;
use crate::widgets::Widget;
use crate::Text;

impl<'a> Widget for AppBar<Text<'a>> {}

// impl<'a> DowncastWidget<Toolbar> for AppBar<Text<'a>> {
//     fn downcast(&self) -> Toolbar {
//         let header = Toolbar::new(self.title, delegate);
//         header
//     }
// }
