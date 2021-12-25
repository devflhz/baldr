use gtk4 as gtk;
use gtk::{ApplicationBuilder, ApplicationWindow, ApplicationWindowBuilder};
use gtk::prelude::*;
use gtk::Application as GApplication;
use crate::{AbstractApplication, AbstractWindow, Application, Window};

impl AbstractApplication<ApplicationWindowBuilder> for Application<ApplicationBuilder> {
    fn builder() -> Application<ApplicationBuilder> {
        Self {
            application: GApplication::builder()
        }
    }

    fn application_id(&mut self, id: &str) -> Application<ApplicationBuilder> {
        self.application = self.application.clone().application_id(id);
        self.clone()
    }

    fn connect_activate(&self, w: Window<ApplicationWindowBuilder>) {
        let app = self.application.clone().build();
        app.connect_activate(move |app| {
            // We create the main window.
            let window = w.window.clone()
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
            window: ApplicationWindow::builder()
        }
    }

    fn title(&mut self, title: &str) -> Window<ApplicationWindowBuilder> {
        self.window = self.window.clone().title(title);
        self.clone()
    }

    fn default_width(&mut self, w: i32) -> Window<ApplicationWindowBuilder> {
        self.window = self.window.clone().default_width(w);
        self.clone()
    }

    fn default_height(&mut self, h: i32) -> Window<ApplicationWindowBuilder> {
        self.window = self.window.clone().default_height(h);
        self.clone()
    }

    fn build(&self) -> ApplicationWindow {
        self.window.clone().build()
    }
}