use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
    widget::Widget,
    AppError,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaLabel as NativeLabel;

pub struct Label {
    native_label: NativeLabel,
}

impl Label {
    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let native_label = NativeLabel::new(text);

        match native_label {
            Ok(native_label) => Ok(Self { native_label }),
            Err(err) => Err(err),
        }
    }

    pub fn get_text(&self) -> String {
        self.native_label.get_text()
    }

    pub fn set_text(&self, text: &'static str) {
        self.native_label.set_text(text);
    }
}

impl Widget for Label {
    fn paint(&mut self, _: &mut Graphics) {}

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }

    fn native_widget(&self) -> Option<&dyn super::cocoa::CocoaWidget> {
        Some(&self.native_label)
    }
}
