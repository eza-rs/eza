use crate::AppError;

use cocoa::{
    appkit::{NSBackingStoreBuffered, NSWindow, NSWindowStyleMask},
    base::{id, nil, NO},
    foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString},
};

pub struct OSXFrame {
    window: id,
}

impl OSXFrame {
    pub fn new(title: &'static str) -> Result<Self, AppError> {
        let window = unsafe {
            let handle = NSWindow::alloc(nil);

            if handle == nil {
                return Err(AppError::AllocFail("failed to allocate NSWindow"));
            }

            let handle = handle.initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO,
            );

            if handle == nil {
                return Err(AppError::InitFail("failed to initialize NSWindow"));
            }

            let handle = handle.autorelease();

            if handle == nil {
                return Err(AppError::InitFail(
                    "failed to enable autorelease for NSWindow",
                ));
            }

            handle
        };

        let title = unsafe {
            let handle = NSString::alloc(nil);

            if handle == nil {
                return Err(AppError::AllocFail("failed to allocate NSString"));
            }

            let handle = handle.init_str(title);

            if handle == nil {
                return Err(AppError::AllocFail("failed to initialize NSString"));
            }

            handle
        };

        unsafe {
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.center();

            window.setTitle_(title);
            window.makeKeyAndOrderFront_(nil);
        }

        Ok(OSXFrame { window })
    }

    pub fn set_title(&mut self, title: &'static str) {
        let title = unsafe {
            let handle = NSString::alloc(nil);

            if handle == nil {
                return;
            }

            let handle = handle.init_str(title);

            if handle == nil {
                return;
            }

            handle
        };

        unsafe {
            self.window.setTitle_(title);
        }
    }
}
