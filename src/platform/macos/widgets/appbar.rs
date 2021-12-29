use cacao::image::{Image, MacSystemIcon};
use cacao::macos::toolbar::{Toolbar, ToolbarItem, ToolbarDelegate, ItemIdentifier};

use crate::widgets::{Widget, Native};
use crate::widgets::appbar::AppBar;

impl Widget for AppBar {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct AppToolbar((ToolbarItem, ToolbarItem));

impl Default for AppToolbar {
    fn default() -> Self {
        AppToolbar(({
            let mut item = ToolbarItem::new("general");
            item.set_title("General");

            let icon = Image::system_icon(MacSystemIcon::PreferencesGeneral, "General");
            item.set_image(icon);
            
            item.set_action(|| {
                // dispatch_ui(Message::SwitchPreferencesToGeneralPane);
            });

            item
        }, {
            let mut item = ToolbarItem::new("advanced");
            item.set_title("Advanced");
            
            let icon = Image::system_icon(MacSystemIcon::PreferencesAdvanced, "Advanced");
            item.set_image(icon);
            
            item.set_action(|| {
                // dispatch_ui(Message::SwitchPreferencesToAdvancedPane);
            });
            
            item
        }))
    }
}

impl ToolbarDelegate for AppToolbar {
    const NAME: &'static str = "PreferencesToolbar";
    
    fn did_load(&mut self, toolbar: Toolbar) {
        toolbar.set_selected("general");
    }

    fn allowed_item_identifiers(&self) -> Vec<ItemIdentifier> {
        vec![ItemIdentifier::Custom("general"), ItemIdentifier::Custom("advanced")]
    }

    fn default_item_identifiers(&self) -> Vec<ItemIdentifier> {
        vec![ItemIdentifier::Custom("general"), ItemIdentifier::Custom("advanced")]
    }

    fn selectable_item_identifiers(&self) -> Vec<ItemIdentifier> {
        vec![ItemIdentifier::Custom("general"), ItemIdentifier::Custom("advanced")]
    }

    fn item_for(&self, identifier: &str) -> &ToolbarItem {
        match identifier {
            "general" => &self.0.0,
            "advanced" => &self.0.1,
            _ => { unreachable!(); }
        }
    }
}

impl Native<AppToolbar> for AppBar {
    fn native(&self) -> AppToolbar {
        let header = AppToolbar::default();
        header
    }
}
