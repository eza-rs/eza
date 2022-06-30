use crate::{
    widget::{Frame, Widget},
    AppError,
};

use cacao::{
    geometry::Rect,
    macos::window::{Window, WindowConfig, WindowStyle, WindowToolbarStyle},
    view::View,
};

pub struct OSXFrame {
    window: Window,
    content_view: View,
}

impl OSXFrame {
    pub fn new(title: &'static str) -> Result<Self, AppError> {
        let window = Window::new(WindowConfig {
            style: WindowStyle::Titled.into(),
            initial_dimensions: Rect::new(0.0, 0.0, Frame::DEFAULT_WIDTH, Frame::DEFAULT_HEIGHT),
            defer: true,
            toolbar_style: WindowToolbarStyle::Automatic,
        });

        let content_view = View::new();

        window.set_title(title);
        window.set_content_view(&content_view);

        window.show();

        Ok(OSXFrame {
            window,
            content_view,
        })
    }

    pub fn set_title(&self, title: &'static str) {
        self.window.set_title(title);
    }

    pub fn add_widget(&self, widget: &dyn Widget) {
        if let Some(native_widget) = widget.native_widget() {
            native_widget.add_to_view(&self.content_view);
        }
    }
}
