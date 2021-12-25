use cacao::macos::{App, AppDelegate};
use cacao::macos::window::Window as NSWindow;

use crate::{AbstractApplication, AbstractWindow, Application, Window};

impl AppDelegate for Window<NSWindow> {
    fn did_finish_launching(&self) {
        self.window.set_content_size(self.properties.default_width, self.properties.default_height);
        self.window.set_title(self.properties.title.as_str());
        self.window.show();
    }
}

#[cfg(target_os = "macos")]
impl AbstractApplication<NSWindow> for Application {
    fn builder() -> Application {
        Self {
            properties: Default::default()
        }
    }

    fn name(&mut self, name: &str) -> Self {
        self.properties.name = name.to_string();
        self.clone()
    }

    fn application_id(&mut self, id: &str) -> Application {
        self.properties.app_id = id.to_string();
        self.clone()
    }

    fn connect_activate(&self, w: Window<NSWindow>) {
        App::new(self.properties.app_id.as_str(), w).run();
    }
}

impl AbstractWindow<NSWindow> for Window<NSWindow> {
    fn builder() -> Window<NSWindow> {
        Window {
            window: NSWindow::default(),
            properties: Default::default()
        }
    }

    fn title(mut self, title: &str) -> Self {
        self.properties.title = title.to_string();
        self
    }

    fn default_width(mut self, w: i32) -> Self {
        self.properties.default_width = w;
        self
    }

    fn default_height(mut self, h: i32) -> Self {
        self.properties.default_height = h;
        self
    }

    fn set_width(mut self, w: i32) -> Self {
        self.properties.width = w;
        self
    }

    fn set_height(mut self, h: i32) -> Self {
        self.properties.height = h;
        self
    }
}