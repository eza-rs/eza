use eza::{
    event::{Event, EventResult},
    widget::{Frame, Label},
    App, AppError,
};

struct MyApp {
    frame: Frame,
    label: Label,
}

impl MyApp {
    fn new() -> Self {
        Self {
            frame: Frame::new().unwrap(),
            label: Label::new("Hello, world!").unwrap(),
        }
    }
}

impl App for MyApp {
    fn init(&mut self) {
        self.frame.set_title("MyApp");

        self.frame.add_widget(&self.label);
    }

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }
}

fn main() -> Result<(), AppError> {
    MyApp::new().run("me.undersquire.HelloWorld")
}
