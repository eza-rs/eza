use cacao::macos::{App as NSApp, AppDelegate};

use crate::app::AppError;

struct CocoaAppDelegate<F: Fn() -> Result<(), AppError> + Send + Sync + 'static> {
    f: F,
}

// TODO: Implement application events
impl<F: Fn() -> Result<(), AppError> + Send + Sync + 'static> AppDelegate for CocoaAppDelegate<F> {
    fn did_finish_launching(&self) {
        NSApp::activate();

        (self.f)().unwrap();
    }
}

pub struct CocoaApp<F: Fn() -> Result<(), AppError> + Send + Sync + 'static> {
    app: NSApp<CocoaAppDelegate<F>>,
}

impl<F: Fn() -> Result<(), AppError> + Send + Sync + 'static> CocoaApp<F> {
    pub fn new(app_id: &'static str, f: F) -> Result<Self, AppError> {
        Ok(Self {
            app: NSApp::new(app_id, CocoaAppDelegate { f }),
        })
    }

    pub fn run(&self) -> Result<(), AppError> {
        self.app.run();

        Ok(())
    }
}
