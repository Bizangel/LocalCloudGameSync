const MINIFIED_HTML_STR: &str = include_str!("../src-ui/dist/index.html");

use std::{cell::RefCell, rc::Rc};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    keyboard::Key,
    window::WindowBuilder,
};
use wry::http::{Response, header::CONTENT_TYPE};
use wry::{
    Rect, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize},
    http::Request,
};

enum UserEvent {
    SampleCommand,
}

pub fn ui_loop_main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let event_proxy = event_loop.create_proxy();

    let handler = move |req: Request<String>| {
        let body = req.body();
        match body.as_str() {
            "sample-command" => {
                let _ = event_proxy.send_event(UserEvent::SampleCommand);
            }
            _ => {}
        }
    };

    let window = WindowBuilder::new()
        .with_title("Wry Minimal Tao")
        .with_inner_size(tao::dpi::LogicalSize::new(800.0, 600.0))
        .with_resizable(true)
        .build(&event_loop)
        .expect("Failed to create window");

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
    #[cfg(debug_assertions)]
    {
        devtool_enabled = true;
    }
    #[cfg(not(debug_assertions))]
    {
        devtool_enabled = false;
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
        .with_url("app://localhost/") // Load from custom protocol
        // .with_html(MINIFIED_HTML_STR)
        .with_ipc_handler(handler);

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    let logical_size = new_size.to_logical::<f64>(window.scale_factor());
                    webview
                        .borrow()
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, 0).into(),
                            size: LogicalSize::new(logical_size.width, logical_size.height).into(),
                        })
                        .unwrap();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let key = event.logical_key;
                    match key {
                        Key::Character("i") => {
                            webview.borrow().open_devtools();
                        }
                        _ => {}
                    }
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::UserEvent(event) => match event {
                UserEvent::SampleCommand => {
                    println!("Sample Command");
                    webview
                        .borrow()
                        .evaluate_script("console.log(\"hello\")")
                        .unwrap();
                }
            },
            _ => {}
        }
    });
}
