use std::ffi::CString;

use gtk::{
    ffi::{
        gtk_widget_show, gtk_widget_show_all, gtk_window_new, gtk_window_set_title, GtkContainer,
        GtkWidget, GtkWindow, GTK_WINDOW_TOPLEVEL, gtk_widget_destroy,
    },
    main_quit,
};

use crate::{gtk_signal_connect, widget::Widget, AppError};

use super::gtk_callback;

pub struct GtkFrame(*mut GtkWidget);

impl GtkFrame {
    pub fn new(title: &'static str) -> Result<Self, AppError> {
        let window = unsafe {
            let win = gtk_window_new(GTK_WINDOW_TOPLEVEL);
            let title = CString::new(title).unwrap();
            gtk_window_set_title(win as *mut GtkWindow, title.as_ptr());
            gtk_widget_show(win);

            gtk_signal_connect!(win, "destroy", gtk_callback(|_| main_quit()));

            win
        };

        if window.is_null() {
            Err(AppError::Unknown)
        } else {
            Ok(Self(window))
        }
    }

    pub fn set_title(&mut self, title: &'static str) {
        unsafe {
            let title = CString::new(title).unwrap();
            gtk_window_set_title(self.0 as *mut GtkWindow, title.as_ptr());
        }
    }

    pub fn add_widget(&mut self, widget: &dyn Widget) {
        if let Some(native_widget) = widget.native_widget() {
            unsafe {
                native_widget.add_to_container(self.0 as *mut GtkContainer);
                gtk_widget_show_all(self.0);
            }
        }
    }
}

impl Drop for GtkFrame
{
	fn drop(&mut self) {
		unsafe { gtk_widget_destroy(self.0); }
	}
}
