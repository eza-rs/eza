use crate::AppError;

use super::CocoaWidget;
use cacao::{layout::Layout, text::Label as NSLabel};

pub struct CocoaLabel {
    label: NSLabel,
}

impl CocoaLabel {
    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let label = NSLabel::new();

        label.set_text(text);

        Ok(Self { label })
    }

    pub fn get_text(&self) -> String {
        self.label.get_text()
    }

    pub fn set_text(&mut self, text: &'static str) {
        self.label.set_text(text);
    }
}

impl CocoaWidget for CocoaLabel {
    fn add_to_view(&self, view: &cacao::view::View) {
        view.add_subview(&self.label)
    }
}
