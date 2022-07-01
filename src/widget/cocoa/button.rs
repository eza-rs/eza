use crate::AppError;

use cacao::{button::Button as NSButton, layout::Layout};

use super::CocoaWidget;

pub struct CocoaButton {
    button: NSButton,
}

impl CocoaButton {
    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let button = NSButton::new(text);

        Ok(Self { button })
    }

    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        self.button.set_action(action);
    }
}

impl CocoaWidget for CocoaButton {
    fn add_to_view(&self, view: &cacao::view::View) {
        view.add_subview(&self.button);
    }
}
