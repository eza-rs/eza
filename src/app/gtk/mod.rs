use gtk::{glib::set_prgname, init, main};

use crate::app::AppError;

pub struct GtkApp;

impl GtkApp {
    pub fn new(app_id: &'static str) -> Result<Self, AppError> {
        match init() {
            Ok(_) => {
                set_prgname(Some(app_id));
                Ok(GtkApp {})
            }

            Err(_) => Err(AppError::Unknown),
        }
    }

    pub fn run(
        &self,
    ) -> Result<(), AppError> {
        main();

        Ok(())
    }
}
