use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
    widget::{NativeWidget, Widget},
    AppError,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaTextBox as NativeTextBox;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use super::gtk::GtkTextBox as NativeTextBox;

/// A textbox [`Widget`]
pub struct TextBox {
    native_textbox: NativeTextBox,
}

impl TextBox {
    /// Creates a new [`TextBox`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let textbox = TextBox::new()?;
    /// ```
    pub fn new() -> Result<Self, AppError> {
        let native_textbox = NativeTextBox::new()?;

        Ok(Self { native_textbox })
    }

    /// Gets the [`TextBox`]'s text.
    ///  
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let mut textbox = TextBox::new()?;
    ///
    /// textbox.set_text("Foo");
    ///
    /// assert_eq!(textbox.get_value(), "Foo".to_string());
    /// ```
    pub fn get_value(&self) -> String {
        self.native_textbox.get_value()
    }

    /// Sets the [`TextBox`]'s text.
    ///
    /// ### Arguments:
    ///
    /// * `text`: The textbox's text.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let mut textbox = TextBox::new()?;
    ///
    /// textbox.set_text("Foo");
    /// assert_eq!(textbox.get_value(), "Foo".to_string());
    /// ```
    pub fn set_text(&mut self, text: &'static str) {
        self.native_textbox.set_text(text);
    }
}

impl Widget for TextBox {
    fn paint(&mut self, _: &mut Graphics) {}

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }

    fn native_widget(&self) -> Option<&dyn NativeWidget> {
        Some(&self.native_textbox)
    }
}
