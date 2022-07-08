// platform modules
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) mod cocoa;

// native app
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use cocoa::CocoaApp as NativeApp;

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
pub use self::gtk::GtkApp as NativeApp;

pub mod event;
pub use eza_proc_macros::eza_app;

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
