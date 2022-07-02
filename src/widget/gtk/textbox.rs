use crate::AppError;

use super::GtkWidget as NativeWidget;
use gtk::ffi::GtkWidget;

pub struct GtkTextBox(*mut GtkWidget);

// TODO: Implement TextBox
impl GtkTextBox {
    pub fn new() -> Result<Self, AppError> {
        todo!()
    }

    pub fn get_value(&self) -> String {
        todo!()
	}

    pub fn set_text(&self, text: &'static str) {
        todo!()
    }
}

impl NativeWidget for GtkTextBox {
    fn add_to_container(&self, container: *mut gtk::ffi::GtkContainer) {
        todo!()
    }
}
