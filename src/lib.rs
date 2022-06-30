mod os;

#[cfg(target_os = "macos")]
use os::macos::OSXApp;

pub mod dialog;
pub mod event;
pub mod graphics;
pub mod widget;

use event::{Event, EventResult};
use graphics::Graphics;
use widget::Frame;

pub trait App {
    fn init(&mut self, frame: &mut Frame);
    fn paint(&mut self, graphics: &mut Graphics);
    fn on_event(&mut self, event: &Event) -> EventResult;
}

#[derive(Debug)]
pub enum AppError {
    Unknown,
    InitFail(&'static str),
    AllocFail(&'static str),
}

pub struct AppDelegate<T: App + Default> {
    app: T,
    frame: Frame,
    // natives
    #[cfg(target_os = "macos")]
    native_app: OSXApp,
}

impl<T: App + Default> AppDelegate<T> {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        let native_app = OSXApp::new().unwrap();

        Self {
            app: T::default(),
            frame: Frame::new().unwrap(),
            native_app,
        }
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        // platform stuff
        // ...

        // init
        self.app.init(&mut self.frame);

        // run application loop
        self.native_app.run()
    }
}
