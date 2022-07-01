use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
    widget::Widget,
    AppError,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaTextBox as NativeTextBox;

pub struct TextBox {
    native_textbox: NativeTextBox,
}

impl TextBox {
    pub fn new() -> Result<Self, AppError> {
        let native_textbox = NativeTextBox::new();

        match native_textbox {
            Ok(native_textbox) => Ok(Self { native_textbox }),
            Err(err) => Err(err),
        }
    }

    pub fn get_value(&self) -> String {
        self.native_textbox.get_value()
    }

    pub fn set_text(&self, text: &'static str) {
        self.native_textbox.set_text(text);
    }
}

impl Widget for TextBox {
    fn paint(&mut self, _: &mut Graphics) {}

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }

    fn native_widget(&self) -> Option<&dyn super::cocoa::CocoaWidget> {
        Some(&self.native_textbox)
    }
}
