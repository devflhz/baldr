use crate::widgets::appbar::AppBar;
use crate::widgets::WidgetTrait;
use crate::Text;

impl<'a> WidgetTrait for AppBar<Text<'a>> {}

// impl<'a> DowncastWidget<Toolbar> for AppBar<Text<'a>> {
//     fn downcast(&self) -> Toolbar {
//         let header = Toolbar::new(self.title, delegate);
//         header
//     }
// }
