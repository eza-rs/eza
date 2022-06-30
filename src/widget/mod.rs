mod button;
mod frame;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
use self::macos::OSXWidget as NativeWidget;

use super::{Event, EventResult, Graphics};

pub use button::Button;
pub use frame::Frame;

pub trait Widget {
    fn paint(&mut self, g: &mut Graphics);
    fn on_event(&mut self, e: &Event) -> EventResult;

    fn native_widget(&self) -> Option<&dyn NativeWidget> {
        None
    }
}
