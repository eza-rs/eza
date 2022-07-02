// platform modules
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) mod cocoa;

// native app
#[cfg(any(target_os = "macos", target_os = "ios"))]
use cocoa::CocoaApp as NativeApp;

// platform modules
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
pub(crate) mod gtk;

//native app
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
use self::gtk::GtkApp as NativeApp;

pub mod event;

use event::{Event, EventResult};

#[derive(Debug)]
pub enum AppError {
    Unknown,
    InitFail(&'static str),
    AllocFail(&'static str),
}

pub trait App {
    fn init(&mut self);
    fn on_event(&mut self, event: &Event) -> EventResult;
}

pub struct AppDelegate<T: App + Default> {
    native_app: NativeApp,
    app: T,
}

impl<T: App + Default> AppDelegate<T> {
    pub fn new(app_id: &'static str) -> Self {
        Self {
            native_app: NativeApp::new(app_id).unwrap(),
            app: T::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        self.app.init();
        self.native_app.run()
    }
}