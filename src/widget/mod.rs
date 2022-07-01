mod button;
mod frame;
mod label;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod cocoa;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use self::cocoa::CocoaWidget as NativeWidget;

use super::{Event, EventResult, Graphics};

pub use button::Button;
pub use frame::Frame;
pub use label::Label;

pub trait Widget {
    fn paint(&mut self, g: &mut Graphics);
    fn on_event(&mut self, e: &Event) -> EventResult;

    fn native_widget(&self) -> Option<&dyn NativeWidget> {
        None
    }
}
