use std::{any::Any};

use cacao::{macos::{AppDelegate, App, window::{Window as NSWindow, WindowDelegate}}, view::View, layout::{Layout, LayoutConstraint}, button::Button as NSButton};

use crate::{AppCreation, widgets::{Widget, window::Window, button::Button, Native}};

#[derive(Debug, Default)]
pub struct Application<'a> {
    pub window: Window<'a>,
}

impl Widget for Application<'static> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AppCreation for Application<'static> {
    fn create(self, bundle_id: &str) {
        App::new(bundle_id, self).run();
    }
}

impl AppDelegate for Application<'static> {
    fn did_finish_launching(&self) {
        App::activate();
        let window = self.window.native();
        window.set_content_size(self.window.default_width, self.window.default_height);
        window.set_title(self.window.title);
        window.show()
    }
}

impl<'a> WindowDelegate for Window<'a> {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: NSWindow) {
        // TODO: This isn't working
        let button = self.body.as_any().downcast_ref::<Button>();
        let mut btn = NSButton::new("text");
        if button.is_some() {
            btn = button.unwrap().native();
            self.view.add_subview(&btn);
        }
        window.set_content_view(&self.view);

        LayoutConstraint::activate(&[
            btn.center_x.constraint_equal_to(&self.view.center_x),
            btn.center_y.constraint_equal_to(&self.view.center_y),
            btn.width.constraint_equal_to_constant(280.)
        ]);
    }
}