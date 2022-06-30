mod frame;

#[cfg(target_os = "macos")]
pub mod macos;

use super::{Event, EventResult, Graphics};

pub use frame::Frame;

pub trait Widget {
    fn paint(&mut self, g: &mut Graphics);
    fn on_event(&mut self, e: &Event) -> EventResult;
}
