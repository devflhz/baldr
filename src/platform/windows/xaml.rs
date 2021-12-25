#![windows_subsystem = "windows"]

use windows::{
    core::*,
    ApplicationModel::Activation::LaunchActivatedEventArgs,
    Win32::System::Com::*,
    UI::Xaml::Controls::Button,
    UI::Xaml::{Application, ApplicationInitializationCallback, Window},
};
use windows as Windows;

#[implement(
    extend Windows::UI::Xaml::Application,
    override OnLaunched
)]
struct MyApp();

#[allow(non_snake_case)]
impl MyApp {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let window = Window::Current()?;
        window.SetContent(Button::new()?)?;
        window.Activate()
    }
}

pub fn run() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    }
    Application::Start(ApplicationInitializationCallback::new(|_| {
        MyApp().new()?;
        Ok(())
    }))
}

