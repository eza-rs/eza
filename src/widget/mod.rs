mod button;
mod frame;
mod label;
mod textbox;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod cocoa;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use self::cocoa::CocoaWidget as NativeWidget;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub mod gtk;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use self::gtk::GtkWidget as NativeWidget;

use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
};

pub use button::Button;
pub use frame::Frame;
pub use label::Label;
pub use textbox::TextBox;

pub trait Widget {
    fn paint(&mut self, g: &mut Graphics);
    fn on_event(&mut self, e: &Event) -> EventResult;

    fn native_widget(&self) -> Option<&dyn NativeWidget> {
        None
    }
}
