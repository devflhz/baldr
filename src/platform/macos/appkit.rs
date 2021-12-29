use std::{any::Any};

use cacao::macos::{AppDelegate, App, window::Window as NSWindow};

use crate::{AppCreation, widgets::{Widget, window::Window}};

#[cfg(target_os = "macos")]
#[derive(Debug, Default)]
pub struct Application {
    pub props: Window,
    pub window: NSWindow
}

impl Widget for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AppCreation for Application {
    fn create(self, bundle_id: &str) {
        App::new(bundle_id, self).run();
    }
}

impl AppDelegate for Application {
    fn did_finish_launching(&self) {
        self.window.set_content_size(self.props.default_width, self.props.default_height);
        self.window.set_title("Mi App");
        self.window.show()
    }
}
