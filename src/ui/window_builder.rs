use std::{cell::RefCell, rc::Rc};
use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wry::{
    Rect, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize},
    http::Request,
};
use wry::{
    WebView,
    http::{Response, header::CONTENT_TYPE},
};

const MINIFIED_HTML_STR: &str = include_str!("../../src-ui/dist/index.html");
use crate::ui::common::{
    UI_INITIAL_SIZE_HEIGHT_PX, UI_INITIAL_SIZE_WIDTH_PX, UI_TITLE_NAME, UIEvent,
};

pub fn build_window_with_webview<F>(
    event_loop: &EventLoop<UIEvent>,
    webview_ipc_handler: F,
) -> (Window, Rc<RefCell<WebView>>)
where
    F: Fn(Request<String>) + 'static,
{
    // This is some huge boilerplate that I have no idea how half of it works.
    // best reference is: https://github.com/tauri-apps/wry/blob/dev/examples/gtk_multiwebview.rs
    let window = WindowBuilder::new()
        .with_title(UI_TITLE_NAME)
        .with_inner_size(tao::dpi::LogicalSize::new(
            UI_INITIAL_SIZE_WIDTH_PX,
            UI_INITIAL_SIZE_HEIGHT_PX,
        ))
        .with_resizable(true)
        .build(&event_loop)
        .expect("Failed to create window");

    window.set_focus();
    window.set_always_on_top(true);

    if let (Some(monitor), window_size) = (window.current_monitor(), window.outer_size()) {
        let monitor_pos = monitor.position();
        let monitor_size = monitor.size();

        let centered_x =
            monitor_pos.x + ((monitor_size.width as i32 - window_size.width as i32) / 2);
        let centered_y =
            monitor_pos.y + ((monitor_size.height as i32 - window_size.height as i32) / 2);

        window.set_outer_position(tao::dpi::PhysicalPosition::new(centered_x, centered_y));
    }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let fixed = {
        use gtk::prelude::*;
        use tao::platform::unix::WindowExtUnix;

        let fixed = gtk::Fixed::new();
        let vbox = window.default_vbox().unwrap();
        vbox.pack_start(&fixed, true, true, 0);
        fixed.show_all();
        fixed
    };

    let devtool_enabled;
    let app_url;
    let disable_right_click_menu_script;
    #[cfg(debug_assertions)]
    {
        use crate::ui::common::VITE_DEV_LOCALHOST_URL;
        devtool_enabled = true;
        app_url = VITE_DEV_LOCALHOST_URL;
        disable_right_click_menu_script = "";
    }
    #[cfg(not(debug_assertions))]
    {
        devtool_enabled = false;
        app_url = "app://localhost";
        disable_right_click_menu_script = r#"""
            document.addEventListener('contextmenu', event => event.preventDefault());
        """#;
    }

    // Get initial size with proper DPI scaling
    let initial_size = window.inner_size().to_logical::<f64>(window.scale_factor());

    // Cross-platform webview build
    let builder = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(initial_size.width, initial_size.height).into(),
        })
        .with_devtools(devtool_enabled)
        .with_custom_protocol("app".into(), move |_, request| {
            let path = request.uri().path();

            if path == "/" || path == "/index.html" {
                Response::builder()
                    .header(CONTENT_TYPE, "text/html")
                    .body(MINIFIED_HTML_STR.as_bytes().into())
                    .unwrap()
            } else {
                Response::builder()
                    .status(404)
                    .body(Vec::new().into())
                    .unwrap()
            }
        })
        .with_initialization_script(disable_right_click_menu_script)
        .with_url(app_url) // Load from custom protocol
        .with_ipc_handler(webview_ipc_handler);

    let webview = {
        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        {
            builder.build(&window).expect("Failed to build webview")
        }

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        {
            use wry::WebViewBuilderExtUnix;
            builder
                .build_gtk(&fixed)
                .expect("Failed to build GTK webview")
        }
    };

    let webview = Rc::new(RefCell::new(webview));

    // GTK-specific size allocation handler
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    {
        use gtk::prelude::*;

        let webview_clone = webview.clone();
        let scale_factor = window.scale_factor();

        fixed.connect_size_allocate(move |_, alloc| {
            let physical_size =
                tao::dpi::PhysicalSize::new(alloc.width() as u32, alloc.height() as u32);
            let logical_size = physical_size.to_logical::<f64>(scale_factor);
            let _ = webview_clone.borrow().set_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: LogicalSize::new(logical_size.width, logical_size.height).into(),
            });
        });
    }

    return (window, webview);
}
