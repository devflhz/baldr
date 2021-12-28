use crate::{Text, Widget};
use crate::widgets::appbar::AppBar;

impl<'a> Default for AppBar<Text<'a>> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for AppBar<Text<'a>> {
    
}

// impl<'a> DowncastWidget<Toolbar> for AppBar<Text<'a>> {
//     fn downcast(&self) -> Toolbar {
//         let header = Toolbar::new(self.title, delegate);
//         header
//     }
// }