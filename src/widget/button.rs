use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
    widget::Widget,
    AppError,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaButton as NativeButton;

pub struct Button {
    native_button: NativeButton,
}

impl Button {
    pub const DEFAULT_WIDTH: f64 = 100.0;
    pub const DEFAULT_HEIGHT: f64 = 50.0;

    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let native_button = NativeButton::new(text);

        match native_button {
            Ok(native_button) => Ok(Self { native_button }),
            Err(err) => Err(err),
        }
    }

    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        self.native_button.set_action(action);
    }
}

impl Widget for Button {
    fn paint(&mut self, _: &mut Graphics) {}

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }

    fn native_widget(&self) -> Option<&dyn super::NativeWidget> {
        Some(&self.native_button)
    }
}
