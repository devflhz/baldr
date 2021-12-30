use gtk4 as gtk;
use gtk::ApplicationWindow;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application as GApplication;
use gtk::prelude::WidgetExt;
use gtk4::prelude::{GtkWindowExt};
use crate::widgets::button::Button;
use crate::widgets::Native;
use crate::{
    AppCreation,
    Application
};

impl AppCreation for Application<'static> {
    fn create(self, bundle_id: &str) {
        let app = GApplication::builder()
            .application_id(bundle_id)
            .build();
        let options = self.window.clone();
        let app_bar = self.window.app_bar.clone();
        let body = self.window.body.clone();
        app.connect_activate(move |app | {
            let window = ApplicationWindow::builder()
                .title(options.title)
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