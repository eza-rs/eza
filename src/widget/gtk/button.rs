use crate::AppError;

use gtk::ffi::{
	GtkWidget,
	GtkContainer,
	gtk_button_new_with_label,
	gtk_container_add, gtk_widget_show_all,
};
use std::ffi::CString;

use super::{GtkWidget as NativeWidget, gtk_callback};
use crate::gtk_signal_connect;

pub struct GtkButton(*mut GtkWidget);

impl GtkButton {
    pub fn new(text: &'static str) -> Result<Self, AppError> {
		let text = CString::new(text).unwrap();

		unsafe {
       		Ok(Self(gtk_button_new_with_label(text.as_ptr())))
		}
    }

    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        unsafe
		{
			gtk_signal_connect!(self.0, "clicked", gtk_callback(|_| action()));
		}
    }
}

impl NativeWidget for GtkButton {
    fn add_to_container(&self, container: *mut GtkContainer) {
        unsafe {
			gtk_container_add(container, self.0);
			gtk_widget_show_all(self.0);
		}
	}
}
