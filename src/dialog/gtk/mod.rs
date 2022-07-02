use super::DialogResult;

use gtk::{
	gdk::ffi::{
		gdk_screen_get_active_window,
		gdk_screen_get_default,
	},
	ffi::{
		GtkWindow,
		GtkDialog as GDialog,
		gtk_message_dialog_new,
		gtk_window_set_title,
		gtk_dialog_run,
		gtk_widget_destroy,
		GTK_DIALOG_DESTROY_WITH_PARENT,
		GTK_MESSAGE_INFO,
		GTK_BUTTONS_OK,
	},
};

use std::ffi::CString;

#[derive(Default)]
pub struct GtkDialog {
    title: &'static str,
    message: &'static str,
}

// TODO: Fix Dialog
impl GtkDialog {
    pub fn set_title(&mut self, title: &'static str) {
        self.title = title;
    }

    pub fn set_message(&mut self, message: &'static str) {
        self.message = message;
    }

    pub fn show(&self) -> DialogResult {
		unsafe {
			let win = gdk_screen_get_active_window(gdk_screen_get_default());
			let msg = CString::new(self.message).unwrap();
			let dialog = gtk_message_dialog_new(
				win as *mut GtkWindow,
				GTK_DIALOG_DESTROY_WITH_PARENT,
				GTK_MESSAGE_INFO,
				GTK_BUTTONS_OK,
				msg.as_ptr()
			);

			let title = CString::new(self.title).unwrap();
			gtk_window_set_title(dialog as *mut GtkWindow, title.as_ptr());
			gtk_dialog_run(dialog as *mut GDialog);
			gtk_widget_destroy(dialog);
		}

        DialogResult::Ok
    }
}
