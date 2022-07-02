// platform modules
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) mod cocoa;

// native app
#[cfg(any(target_os = "macos", target_os = "ios"))]
use cocoa::CocoaApp as NativeApp;

// platform modules
#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub(crate) mod gtk;

//native app
#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use self::gtk::GtkApp as NativeApp;

pub mod event;

use event::{Event, EventResult};

#[derive(Debug)]
pub enum AppError {
    Unknown,
    InitFail(&'static str),
    AllocFail(&'static str),
}

/// An application.
pub trait App {
    /// Initializes the [`App`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let mut app = MyApp::default();
    /// app.init();
    /// ```
    fn init(&mut self);

    // TODO: Document on_event when events are finished
    fn on_event(&mut self, event: &Event) -> EventResult;
}

/// An application Delegate.
///
/// It is used to run an [`App`].
pub struct AppDelegate<T: App + Default> {
    native_app: NativeApp,
    app: T,
}

impl<T: App + Default> AppDelegate<T> {
    /// Creates a new [`AppDelegate`] for an [`App`] of type `T`.
    ///
    /// ### Arguments:
    ///
    /// * `app_id`: The Application id (eg. `"com.example.app"`).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let delegate = AppDelegate<T>::new("com.foo.Bar");
    /// ```
    pub fn new(app_id: &'static str) -> Self {
        Self {
            native_app: NativeApp::new(app_id).unwrap(),
            app: T::default(),
        }
    }

    /// Runs the [`AppDelegate`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let mut delegate = AppDelegate<T>::new("com.foo.Bar");
    /// let result = delegate.run();
    /// ```
    pub fn run(&mut self) -> Result<(), AppError> {
        str::parse::<u64>("").unwrap();
        self.app.init();
        self.native_app.run()
    }
}
