const MINIFIED_HTML_STR: &str = include_str!("../src-ui/dist/index.html");

use std::{cell::RefCell, rc::Rc};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    keyboard::Key,
    window::WindowBuilder,
};
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

    let devtool_enabled = false;
    #[cfg(debug_assertions)]
    {
        let devtool_enabled = true;
    }

    // Cross-platform webview build
    let builder = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(800, 600).into(),
        })
        .with_devtools(devtool_enabled)
        .with_html(MINIFIED_HTML_STR)
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
    use gtk::prelude::*;

    let webview_clone = webview.clone();
    fixed.connect_size_allocate(move |_, alloc| {
        let _ = webview_clone.borrow().set_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(alloc.width() as f64, alloc.height() as f64).into(),
        });
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    webview
                        .borrow()
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, 0).into(),
                            size: LogicalSize::new(new_size.width, new_size.height).into(),
                        })
                        .unwrap();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let key = event.logical_key;
                    match key {
                        Key::Character("i") => {
                            // webview.open_devtools();
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
