use std::ffi::CString;

use gtk::{
	main_quit,
	ffi::{
		GtkWidget,
		GtkWindow,
		GtkContainer,
		gtk_window_new,
		gtk_window_set_title,
		gtk_widget_show,
		gtk_widget_show_all,
		GTK_WINDOW_TOPLEVEL,
	}
};

use crate::{
	gtk_signal_connect,
    widget::Widget,
    AppError,
};

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
    
		if window == std::ptr::null_mut()
		{
			Err(AppError::Unknown)
		}
		else
		{
			Ok(Self(window))
		}
    }

    pub fn set_title(&self, title: &'static str) {
        unsafe
		{
			let title = CString::new(title).unwrap();
			gtk_window_set_title(self.0 as *mut GtkWindow, title.as_ptr());
		}
    }

    pub fn add_widget(&self, widget: &dyn Widget) {
        if let Some(native_widget) = widget.native_widget() {
            native_widget.add_to_container(self.0 as *mut GtkContainer);
			unsafe { gtk_widget_show_all(self.0); }
        }
    }
}
