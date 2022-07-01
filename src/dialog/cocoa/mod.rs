use super::DialogResult;

use cacao::macos::Alert;

#[derive(Default)]
pub struct CocoaDialog {
    title: &'static str,
    message: &'static str,
}

impl CocoaDialog {
    pub fn set_title(&mut self, title: &'static str) {
        self.title = title;
    }

    pub fn set_message(&mut self, message: &'static str) {
        self.message = message;
    }

    pub fn show(&self) -> DialogResult {
        Alert::new(self.title, self.message).show();

        DialogResult::Ok
    }
}
