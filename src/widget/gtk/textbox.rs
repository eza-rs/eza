use crate::AppError;

use super::GtkWidget as NativeWidget;
use gtk::{
    ffi::{
        gtk_container_add, gtk_text_buffer_get_bounds, gtk_text_buffer_get_text,
        gtk_text_buffer_set_text, gtk_text_view_get_buffer, gtk_text_view_new, gtk_widget_show_all,
        GtkTextIter, GtkTextView, GtkWidget,
    },
    glib::ffi::g_free,
};

use std::ffi::{c_void, CStr, CString};

pub struct GtkTextBox(*mut GtkWidget);

impl GtkTextBox {
    pub fn new() -> Result<Self, AppError> {
        unsafe { Ok(Self(gtk_text_view_new())) }
    }

    pub fn get_value(&self) -> String {
        unsafe {
            let buf = gtk_text_view_get_buffer(self.0 as *mut GtkTextView);
            let mut start: GtkTextIter =
                std::mem::transmute([0u8; std::mem::size_of::<GtkTextIter>()]);
            let mut end: GtkTextIter =
                std::mem::transmute([0u8; std::mem::size_of::<GtkTextIter>()]);

            gtk_text_buffer_get_bounds(buf, &mut start, &mut end);

            let strptr = gtk_text_buffer_get_text(buf, &start, &end, 1);
            let str = CStr::from_ptr(strptr);
            let str = str.to_str().unwrap().to_string();

            g_free(strptr as *mut c_void);

            str
        }
    }

    pub fn set_text(&mut self, text: &'static str) {
        let str = CString::new(text).unwrap();

        unsafe {
            let buf = gtk_text_view_get_buffer(self.0 as *mut GtkTextView);
            gtk_text_buffer_set_text(buf, str.as_ptr(), -1);
        }
    }
}

impl NativeWidget for GtkTextBox {
    unsafe fn add_to_container(&self, container: *mut gtk::ffi::GtkContainer) {
        gtk_container_add(container, self.0);
        gtk_widget_show_all(self.0);
    }
}
