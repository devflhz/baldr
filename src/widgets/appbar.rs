use super::text::Text;


#[derive(Debug, Default, Clone)]
pub struct AppBar<'a> {
    pub title: Text<'a>,
}

impl<'a> AppBar<'a> {
    pub fn new(title: &'a str) -> Option<Self> {
        Some(
            Self {
                title: Text(title)
            }
        )
    }
}
