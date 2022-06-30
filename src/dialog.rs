#[cfg(target_os = "macos")]
use super::macos::OSXDialog;

pub enum DialogResult {
    Ok,
    Error,
}

/// WARNING: This currently cannot be used inside `init`!
#[derive(Default)]
pub struct DialogBuilder {
    #[cfg(target_os = "macos")]
    native_dialog: OSXDialog,
}

impl DialogBuilder {
    pub fn new() -> Self {
        Self {
            native_dialog: Default::default(),
        }
    }

    pub fn title(&mut self, title: &'static str) -> &mut Self {
        self.native_dialog.set_title(title);
        self
    }

    pub fn message(&mut self, message: &'static str) -> &mut Self {
        self.native_dialog.set_message(message);
        self
    }

    pub fn show(&mut self) -> DialogResult {
        self.native_dialog.show()
    }
}
