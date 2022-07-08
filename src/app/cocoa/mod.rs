use cacao::macos::{App as NSApp, AppDelegate};

use crate::app::AppError;

struct CocoaAppDelegate;

// TODO: Implement application events
impl AppDelegate for CocoaAppDelegate {
    fn did_finish_launching(&self) {
        NSApp::activate();
    }
}

pub struct CocoaApp {
    app: NSApp<CocoaAppDelegate>,
}

impl CocoaApp {
    pub fn new(app_id: &'static str) -> Result<Self, AppError> {
        Ok(Self {
            app: NSApp::new(app_id, CocoaAppDelegate),
        })
    }

    pub fn run(&self) -> Result<(), AppError> {
        self.app.run();

        Ok(())
    }
}
