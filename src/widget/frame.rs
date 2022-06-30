use super::{Event, EventResult, Graphics, Widget};
use crate::AppError;

#[cfg(target_os = "macos")]
use super::macos::OSXFrame;

pub struct Frame {
    // native handles
    #[cfg(target_os = "macos")]
    native_frame: OSXFrame,
}

impl Frame {
    pub const DEFAULT_TITLE: &'static str = "Application";
    pub const DEFAULT_WIDTH: f64 = 640.0;
    pub const DEFAULT_HEIGHT: f64 = 480.0;

    pub fn new() -> Result<Self, AppError> {
        #[cfg(target_os = "macos")]
        let native_frame = OSXFrame::new(Frame::DEFAULT_TITLE);

        match native_frame {
            Ok(native_frame) => Ok(Self { native_frame }),
            Err(err) => Err(err),
        }
    }

    pub fn set_title(&self, title: &'static str) {
        self.native_frame.set_title(title);
    }

    pub fn add_widget(&self, widget: &dyn Widget) {
        self.native_frame.add_widget(widget);
    }
}

impl Widget for Frame {
    fn paint(&mut self, _: &mut Graphics) {}

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }
}
