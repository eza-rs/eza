use crate::AppError;

use gtk::ffi::{
    gtk_button_new_with_label, gtk_widget_destroy, gtk_container_add, gtk_widget_show_all, GtkContainer, GtkWidget,
};
use std::ffi::CString;

use super::{gtk_callback, GtkWidget as NativeWidget};
use crate::gtk_signal_connect;

pub struct GtkButton(*mut GtkWidget);

impl GtkButton {
    pub fn new(text: &'static str) -> Result<Self, AppError> {
        let text = CString::new(text).unwrap();

        unsafe { Ok(Self(gtk_button_new_with_label(text.as_ptr()))) }
    }

    pub fn set_action<F: Fn() + Send + Sync + 'static>(&mut self, action: F) {
        unsafe {
            gtk_signal_connect!(self.0, "clicked", gtk_callback(|_| action()));
        }
    }
}

impl Drop for GtkButton
{
	fn drop(&mut self) {
		unsafe { gtk_widget_destroy(self.0); }
	}
}

impl NativeWidget for GtkButton {
    unsafe fn add_to_container(&self, container: *mut GtkContainer) {
        gtk_container_add(container, self.0);
        gtk_widget_show_all(self.0);
    }
}
