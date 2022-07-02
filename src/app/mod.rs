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

/// Abstracts over general errors from the underlying backends.
#[derive(Debug)]
pub enum AppError {
    /// If you receive this error value, it means the underlying backend error
    /// is undocumented and/or no [`AppError`] value/translation exists for it yet.
    /// Please make an issue on the GitHub repository with more details.
    Unknown,
    /// Something regarding calls to the underlying backend failed (see error message).
    InitFail(&'static str),
    /// The underlying backend failed to allocate (see error message).
    AllocFail(&'static str),
}

/// An application.
pub trait App {
    /// Invoked by the [`AppDelegate`].
    ///
    /// **WARNING:** This function is purely for initializing data fields of
    /// your [`App`] object. Certain features (such as [`DialogBuilder`]) can not
    /// be used within this function, as the underlying backend is not guarenteed to be
    /// fully initialized yet.
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
        self.app.init();
        self.native_app.run()
    }
}
