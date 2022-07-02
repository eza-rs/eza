use eza::{
    event::{Event, EventResult},
    widget::{Frame, Label},
    App, AppDelegate, AppError,
};

struct MyApp {
    frame: Frame,
    label: Label,
}

impl Default for MyApp {
    fn default() -> Self {
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
    AppDelegate::<MyApp>::new("me.eza-rs.HelloWorld").run()
}