impl<'a> Widget<String> for Text<'a> {
    fn widget(&self) -> String {
        self.0.to_string()
    }
}
