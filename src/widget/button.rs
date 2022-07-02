use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
    widget::{NativeWidget, Widget},
    AppError,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaButton as NativeButton;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use super::gtk::GtkButton as NativeButton;

/// A button [`Widget`].
pub struct Button {
    native_button: NativeButton,
}

impl Button {
    pub const DEFAULT_WIDTH: f64 = 100.0;
    pub const DEFAULT_HEIGHT: f64 = 50.0;

    /// Creates a new [`Button`].
    ///
    /// ### Arguments:
    ///
    /// * `text`: The button's label's text.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the [`Button`] failed to initialize.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let button = Button::new("Hello World!")?;
    /// ```
    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let native_button = NativeButton::new(text)?;

        Ok(Self { native_button })
    }

    /// Sets the callback called when the [`Button`] is pressed.
    ///
    /// ### Arguments:
    ///
    /// * `action`: The callback function.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let button = Button::new("Foo")?;
    /// button.set_action(|| println!("Bar"));
    /// ```
    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        self.native_button.set_action(action);
    }
}

impl Widget for Button {
    fn paint(&mut self, _: &mut Graphics) {}

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }

    fn native_widget(&self) -> Option<&dyn NativeWidget> {
        Some(&self.native_button)
    }
}
