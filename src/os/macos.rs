use cacao::macos::{Alert, App as NSApp, AppDelegate};

use crate::{dialog::DialogResult, AppError};

struct OSXAppDelegate;

impl AppDelegate for OSXAppDelegate {
    fn did_finish_launching(&self) {
        NSApp::activate();
    }
}

pub struct OSXApp {
    app: NSApp<OSXAppDelegate>,
}

impl OSXApp {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            app: NSApp::new("", OSXAppDelegate),
        })
    }

    pub fn run(&self) -> Result<(), AppError> {
        self.app.run();

        Ok(())
    }
}

#[derive(Default)]
pub struct OSXDialog {
    title: &'static str,
    message: &'static str,
}

impl OSXDialog {
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
