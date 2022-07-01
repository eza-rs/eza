mod os;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) use os::cocoa::{self, CocoaApp as NativeApp};

pub mod dialog;
pub mod event;
pub mod graphics;
pub mod widget;

use event::{Event, EventResult};
use graphics::Graphics;

#[derive(Debug)]
pub enum AppError {
    Unknown,
    InitFail(&'static str),
    AllocFail(&'static str),
}

pub trait App {
    fn init(&mut self);
    fn on_event(&mut self, event: &Event) -> EventResult;

    fn run(&mut self, app_id: &'static str) -> Result<(), AppError> {
        let native_app = NativeApp::new(app_id).unwrap();

        self.init();

        native_app.run()
    }
}
