use cacao::macos::{Alert, App as NSApp, AppDelegate};

use crate::{dialog::DialogResult, AppError};

struct CocoaAppDelegate;

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

#[derive(Default)]
pub struct CocoaDialog {
    title: &'static str,
    message: &'static str,
}

impl CocoaDialog {
    pub fn set_title(&mut self, title: &'static str) {
        self.title = title;
    }

    pub fn set_message(&mut self, message: &'static str) {
        self.message = message;
    }

    pub fn show(&self) -> DialogResult {
        Alert::new(self.title, self.message).show();

        DialogResult::Ok
    }
}
