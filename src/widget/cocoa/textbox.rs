use crate::AppError;

use super::CocoaWidget;
use cacao::{input::TextField as NSTextField, layout::Layout};

pub struct CocoaTextBox {
    textbox: NSTextField,
}

impl CocoaTextBox {
    pub fn new() -> Result<Self, AppError> {
        let textbox = NSTextField::new();

        Ok(Self { textbox })
    }

    pub fn get_value(&self) -> String {
        self.textbox.get_value()
    }

    pub fn set_text(&self, text: &'static str) {
        self.textbox.set_text(text);
    }
}

impl CocoaWidget for CocoaTextBox {
    fn add_to_view(&self, view: &cacao::view::View) {
        view.add_subview(&self.textbox)
    }
}
