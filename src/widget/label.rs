use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
    widget::{Widget, NativeWidget},
    AppError,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaLabel as NativeLabel;

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
use super::gtk::GtkLabel as NativeLabel;

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

    fn native_widget(&self) -> Option<&dyn NativeWidget> {
        Some(&self.native_label)
    }
}
