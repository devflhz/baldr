use gtk4 as gtk;
use gtk::ApplicationWindow;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application as GApplication;
use gtk::prelude::WidgetExt;
use gtk4::prelude::{GtkWindowExt};
use crate::Application;
use crate::widgets::button::Button;
use crate::widgets::Native;

#[derive(Debug, Clone)]
pub struct Application {
    pub app_id: String,
    pub window: Window,
}

impl AppCreation for Application {
    fn create(&self) {
        let app = GApplication::builder()
            .application_id(self.app_id.as_str())
            .build();
        let options = self.home.clone();
        let app_bar = self.home.app_bar.clone();
        let body = self.home.body.clone();
        app.connect_activate(move |app | {
            let window = ApplicationWindow::builder()
                .title(options.title.as_str())
                .default_height(options.default_height)
                .default_width(options.default_width)
                .application(app)
                .build();
            if app_bar.is_some() {
                window.set_titlebar(Some(
                    &app_bar.as_ref().unwrap().native()
                ));
            }
            let button = body.as_any().downcast_ref::<Button>();
            if button.is_some() {
                window.set_child(Some(
                    &button.unwrap().native()
                ))
            }
            window.show();
        });
        app.run();
    }
}