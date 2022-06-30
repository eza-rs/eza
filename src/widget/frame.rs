use super::{Event, EventResult, Graphics, Widget};
use crate::AppError;

#[cfg(target_os = "macos")]
use super::macos::OSXFrame;

pub struct Frame {
    pub(crate) widgets: Vec<Box<dyn Widget>>,
    // native handles
    #[cfg(target_os = "macos")]
    native_frame: OSXFrame,
}

impl Frame {
    pub const DEFAULT_TITLE: &'static str = "Application";

    pub fn new() -> Result<Self, AppError> {
        #[cfg(target_os = "macos")]
        let native_frame = OSXFrame::new(Frame::DEFAULT_TITLE);

        match native_frame {
            Ok(native_frame) => Ok(Self {
                widgets: Vec::new(),
                native_frame,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn set_title(&mut self, title: &'static str) {
        self.native_frame.set_title(title);
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Frame {
    fn paint(&mut self, g: &mut Graphics) {
        for widget in self.widgets.iter_mut() {
            widget.paint(g);
        }
    }

    fn on_event(&mut self, e: &Event) -> EventResult {
        for widget in self.widgets.iter_mut() {
            match widget.on_event(e) {
                EventResult::Skip => return EventResult::Skip,
                _ => {}
            }
        }

        EventResult::None
    }
}
