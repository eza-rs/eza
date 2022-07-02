use crate::{
    event::{Event, EventResult},
    graphics::Graphics,
    widget::{NativeWidget, Widget},
    AppError,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::cocoa::CocoaLabel as NativeLabel;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use super::gtk::GtkLabel as NativeLabel;

/// A label [`Widget`].
pub struct Label {
    native_label: NativeLabel,
}

impl Label {
    /// Creates a new [`Label`].
    ///
    /// ### Arguments:
    ///
    /// * `text`: The label's text.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let label = Label::new("Hello, world!")?;
    /// ```
    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let native_label = NativeLabel::new(text)?;

        Ok(Self { native_label })
    }

    /// Gets the [`Label`]'s text.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let label = Label::new("Hello, world!")?;
    ///
    /// assert_eq!(label.get_text(), "Hello World!".to_string());
    /// ```
    pub fn get_text(&self) -> String {
        self.native_label.get_text()
    }

    /// Sets the [`Label`]'s text.
    ///
    /// ### Arguments:
    ///
    /// * `text`: The label's text.
    ///  
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let mut label = Label::new("Foo")?;
    ///
    /// label.set_text("Bar");
    ///
    /// assert_eq!(label.get_text(), "Bar".to_string());
    /// ```
    pub fn set_text(&mut self, text: &'static str) {
        self.native_label.set_text(text);
    }
}

impl Widget for Label {
    fn paint(&mut self, _: &mut Graphics) {}

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }

    fn native_widget(&self) -> Option<&dyn NativeWidget> {
        Some(&self.native_label)
    }
}
