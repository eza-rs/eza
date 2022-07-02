use crate::AppError;

use super::GtkWidget as NativeWidget;
use gtk::ffi::{
    gtk_container_add, gtk_label_get_text, gtk_label_new, gtk_label_set_text, gtk_widget_show,
    GtkContainer, GtkLabel as GLabel, GtkWidget,
};

use std::ffi::{CStr, CString};

pub struct GtkLabel(*mut GtkWidget);

impl GtkLabel {
    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let text = CString::new(text).unwrap();
        let label = unsafe { gtk_label_new(text.as_ptr()) };

        Ok(Self(label))
    }

    pub fn get_text(&self) -> String {
        unsafe {
            CStr::from_ptr(gtk_label_get_text(self.0 as *mut GLabel) as *mut _)
                .to_str()
                .unwrap()
                .to_string()
        }
    }

    pub fn set_text(&mut self, text: &'static str) {
        let text = CString::new(text).unwrap();
        unsafe {
            gtk_label_set_text(self.0 as *mut GLabel, text.as_ptr());
        }
    }
}

impl NativeWidget for GtkLabel {
    unsafe fn add_to_container(&self, container: *mut GtkContainer) {
        gtk_container_add(container, self.0);
        gtk_widget_show(self.0);
    }
}
