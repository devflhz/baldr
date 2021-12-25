#[cfg(target_os = "windows")]
use windows::{
    Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

use crate::{AbstractApplication, Application, AbstractWindow, Window, WindowProperties};

impl AbstractApplication<WindowProperties> for Application {
    fn builder() -> Self {
        Self {
            properties: Default::default(),
        }
    }

    fn name(&mut self, name: &str) -> Self {
        self.properties.name = name.to_string();
        self.clone()
    }

    fn application_id(&mut self, id: &str) -> Self {
        self.properties.app_id = id.to_string();
        self.clone()
    }

    fn connect_activate(&self, window: Window<WindowProperties>) {
        unsafe {
            let instance = GetModuleHandleA(None);
            debug_assert!(instance != 0);

            let window_class = "window";

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW),
                hInstance: instance,
                lpszClassName: PSTR(b"window\0".as_ptr() as _),

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            CreateWindowExA(
                Default::default(),
                window_class,
                if window.properties.title.is_empty() {
                    "Title"
                } else {
                    window.properties.title.as_str()
                },
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                if window.properties.default_width == 0 {
                    CW_USEDEFAULT
                } else {
                    window.properties.default_width
                },
                if window.properties.default_height == 0 {
                    CW_USEDEFAULT
                } else {
                    window.properties.default_height
                },
                None,
                None,
                instance,
                std::ptr::null_mut(),
            );

            let mut message = MSG::default();

            while GetMessageA(&mut message, 0, 0, 0).into() {
                DispatchMessageA(&mut message);
            }
        }
    }
}

impl AbstractWindow<WindowProperties> for Window<WindowProperties> {
    fn builder() -> Self {
        Self {
            window: Default::default(),
            properties: Default::default(),
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

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, std::ptr::null());
                0
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}