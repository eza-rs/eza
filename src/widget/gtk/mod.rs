mod button;
mod frame;
mod label;
mod textbox;

pub use button::*;
pub use frame::*;
pub use label::*;
pub use textbox::*;

use gtk::ffi::GtkContainer;

use std::ffi::c_void;

unsafe extern "C" fn gtk_trampoline<F: Fn(*mut gtk::ffi::GtkWidget)>(widget: *mut gtk::ffi::GtkWidget, f: *mut c_void)
{
	let f = &mut *(f as *mut F);
	f(widget);
}

pub unsafe fn gtk_callback<F: Fn(*mut gtk::ffi::GtkWidget)>(mut f: F) -> (unsafe extern "C" fn(), *mut c_void)
{
	use std::mem::transmute;
	(transmute(gtk_trampoline::<F> as *const ()), &mut f as *mut _ as *mut c_void)
}

#[macro_export]
macro_rules! gtk_signal_connect {
	($inst:expr, $sign:expr, $callback:expr) => {
		{
			use gtk::glib::gobject_ffi::{
				GObject,
				g_signal_connect_data,
			};
			use std::ffi::CString;

			let sign = CString::new($sign).unwrap();
			let (f, data) = $callback;
			g_signal_connect_data($inst as *mut GObject, sign.as_ptr(), Some(f), data, None, 0)
		}
	};
}

pub trait GtkWidget {
    fn add_to_container(&self, container: *mut GtkContainer);
}
