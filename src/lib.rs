mod os;

#[cfg(target_os = "macos")]
pub(crate) use os::macos;

#[cfg(target_os = "macos")]
use os::macos::OSXApp;

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

    fn run(&mut self) -> Result<(), AppError> {
        #[cfg(target_os = "macos")]
        let native_app = OSXApp::new().unwrap();

        self.init();

        native_app.run()
    }
}
