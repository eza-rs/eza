use crate::{
    widget::{Frame, Widget},
    AppError,
};

use cacao::{
    macos::window::{Window, WindowConfig, WindowStyle},
    view::View,
};

pub struct CocoaFrame {
    window: Window,
    content_view: View,
}

impl CocoaFrame {
    pub fn new(title: &'static str) -> Result<Self, AppError> {
        let mut window_config = WindowConfig::default();

        window_config.set_styles(&[
            WindowStyle::Closable,
            WindowStyle::Miniaturizable,
            WindowStyle::Titled,
        ]);
        window_config.set_initial_dimensions(0.0, 0.0, Frame::DEFAULT_WIDTH, Frame::DEFAULT_HEIGHT);

        let window = Window::new(window_config);

        let content_view = View::new();

        window.set_title(title);
        window.set_content_view(&content_view);

        window.show();

        Ok(Self {
            window,
            content_view,
        })
    }

    pub fn set_title(&mut self, title: &'static str) {
        self.window.set_title(title);
    }

    pub fn add_widget(&mut self, widget: &dyn Widget) {
        if let Some(native_widget) = widget.native_widget() {
            native_widget.add_to_view(&self.content_view);
        }
    }
}
