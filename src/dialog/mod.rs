// platform modules
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) mod cocoa;

// native dialog
#[cfg(any(target_os = "macos", target_os = "ios"))]
use cocoa::CocoaDialog as NativeDialog;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub(crate) mod gtk;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use self::gtk::GtkDialog as NativeDialog;

/// **NOTE:** Currently unused!
pub enum DialogResult {
    Ok,
    Error,
}

/// A dialog builder.
///
/// **WARNING**: This currently cannot be used inside `init`!
#[derive(Default)]
pub struct DialogBuilder {
    native_dialog: NativeDialog,
}

impl DialogBuilder {
    /// Creates a new [`DialogBuilder`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// DialogBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            native_dialog: Default::default(),
        }
    }

    /// Sets the title of the [`DialogBuilder`].
    ///
    /// ### Arguments:
    ///
    /// * `title`: The title of the dialog.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// DialogBuilder::new().title("Hello, world!").show();
    /// ```
    pub fn title(&mut self, title: &'static str) -> &mut Self {
        self.native_dialog.set_title(title);
        self
    }

    /// Sets the message of the [`DialogBuilder`]
    ///
    /// ### Arguments:
    ///
    /// * `message`: The title of the dialog.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// DialogBuilder::new().message("Hello, world!").show();
    /// ```
    pub fn message(&mut self, message: &'static str) -> &mut Self {
        self.native_dialog.set_message(message);
        self
    }

    /// Shows the dialog builded by the [`DialogBuilder`]
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// DialogBuilder::new().show();
    /// ```
    pub fn show(&self) -> DialogResult {
        self.native_dialog.show()
    }
}
