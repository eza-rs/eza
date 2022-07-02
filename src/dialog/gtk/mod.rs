use super::DialogResult;

use gtk::{
    ffi::{
        gtk_dialog_run, gtk_message_dialog_new, gtk_widget_destroy, gtk_window_list_toplevels,
        gtk_window_set_title, GtkDialog as GDialog, GtkWindow, GTK_BUTTONS_OK,
        GTK_DIALOG_DESTROY_WITH_PARENT, GTK_MESSAGE_INFO,
    },
    glib::{
        ffi::{g_list_first, g_list_free},
        gobject_ffi::{g_object_ref, g_object_unref},
        object::GObject,
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
            let wins = gtk_window_list_toplevels();
            let win = g_object_ref(g_list_first(wins) as *mut GObject);
            g_list_free(wins);

            let msg = CString::new(self.message).unwrap();
            let dialog = gtk_message_dialog_new(
                win as *mut GtkWindow,
                GTK_DIALOG_DESTROY_WITH_PARENT,
                GTK_MESSAGE_INFO,
                GTK_BUTTONS_OK,
                msg.as_ptr(),
            );

            let title = CString::new(self.title).unwrap();
            gtk_window_set_title(dialog as *mut GtkWindow, title.as_ptr());
            gtk_dialog_run(dialog as *mut GDialog);
            gtk_widget_destroy(dialog);
            g_object_unref(win);
        }

        DialogResult::Ok
    }
}
