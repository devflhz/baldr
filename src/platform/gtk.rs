use gtk4 as gtk;
use gtk::{ApplicationWindow, ApplicationWindowBuilder};
use gtk::prelude::*;
use gtk::Application as GApplication;
use crate::{AbstractApplication, AbstractWindow, Application, Window};

impl AbstractApplication<ApplicationWindowBuilder> for Application {
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

    fn connect_activate(&self, w: Window<ApplicationWindowBuilder>) {
        let app = GApplication::builder()
            .application_id(self.properties.app_id.as_str())
            .build();
        app.connect_activate(move |app| {
            // We create the main window.
            let window = w.window.clone()
                .title(w.properties.title.as_str())
                .default_width(w.properties.default_width)
                .default_height(w.properties.default_height)
                .application(app)
                .build();

            // Show the window.
            window.show();
        });
        app.run();
    }
}

impl AbstractWindow<ApplicationWindow> for Window<ApplicationWindowBuilder> {
    fn builder() -> Window<ApplicationWindowBuilder> {
        Window {
            window: ApplicationWindow::builder(),
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