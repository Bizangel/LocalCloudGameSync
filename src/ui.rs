const MINIFIED_HTML_STR: &str = include_str!("../src-ui/dist/index.html");

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    Rect, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize},
};

pub fn ui_loop_main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
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

    // Cross-platform webview build
    let builder = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(800, 600).into(),
        })
        .with_html(MINIFIED_HTML_STR);

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                webview
                    .set_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: LogicalSize::new(new_size.width, new_size.height).into(),
                    })
                    .unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
