use crate::{widget::Widget, AppError};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaFrame as NativeFrame;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use super::gtk::GtkFrame as NativeFrame;

pub struct Frame {
    // native handles
    native_frame: NativeFrame,
}

/// A window
impl Frame {
    pub const DEFAULT_TITLE: &'static str = "Application";
    pub const DEFAULT_WIDTH: f64 = 640.0;
    pub const DEFAULT_HEIGHT: f64 = 480.0;

    /// Creates a new [`AppDelegate`] for an [`App`] of type `T`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the [`Frame`] failed to initialize.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let frame = Frame::new()?;
    /// ```
    pub fn new() -> Result<Self, AppError> {
        let native_frame = NativeFrame::new(Frame::DEFAULT_TITLE)?;

        Ok(Self { native_frame })
    }

    /// Sets the title of the [`Frame`].
    ///
    /// ### Arguments:
    ///
    /// * `title`: The title of the frame.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let mut frame = Frame::new()?;
    /// frame.set_title("Foo");
    /// ```
    pub fn set_title(&self, title: &'static str) {
        self.native_frame.set_title(title);
    }

    /// Adds a widget to the [`Frame`].
    ///
    /// ### Arguments:
    ///
    /// * `widget`: The widget to add.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let mut button = Button::new()?;
    /// let mut frame = Frame::new()?;
    /// frame.add_widget(&button);
    /// ```
    pub fn add_widget(&self, widget: &dyn Widget) {
        self.native_frame.add_widget(widget);
    }
}
