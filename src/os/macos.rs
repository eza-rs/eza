use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicyRegular, NSRunningApplication,
    },
    base::{id, nil},
    foundation::NSAutoreleasePool,
};

use crate::AppError;

pub struct OSXApp {
    pool: id,
    app_handle: id,
    current_app: id,
}

impl OSXApp {
    pub fn new() -> Result<Self, AppError> {
        let pool = unsafe {
            let handle = NSAutoreleasePool::new(nil);

            if handle == nil {
                return Err(AppError::AllocFail("failed to allocate NSAutoreleasePool"));
            }

            handle
        };

        let app_handle = unsafe {
            let handle = NSApp();

            if handle == nil {
                return Err(AppError::AllocFail("failed to allocate NSApp"));
            }

            handle
        };

        let current_app = unsafe {
            let handle = NSRunningApplication::currentApplication(nil);

            if handle == nil {
                return Err(AppError::AllocFail(
                    "failed to allocate NSRunningApplication",
                ));
            }

            handle
        };

        unsafe {
            if !app_handle.setActivationPolicy_(NSApplicationActivationPolicyRegular) {
                return Err(AppError::InitFail("failed to initialize activation policy"));
            }

            if !current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps) {
                return Err(AppError::InitFail(
                    "failed to initialize activation options",
                ));
            }
        }

        Ok(Self {
            pool,
            app_handle,
            current_app,
        })
    }

    pub fn run(&mut self) -> Result<(), AppError> {
        unsafe {
            self.app_handle.run();
        }

        Ok(())
    }
}

// TODO: Implement this
pub struct OSXDialog {}
