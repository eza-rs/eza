use eza::{
    event::{Event, EventResult},
    widget::{Frame, Label},
    App, AppDelegate, AppError,
};

struct MyApp {
    frame: Frame,
    txt: Label,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            frame: Frame::new().unwrap(),
            txt: Label::new("Hello, world!").unwrap(),
        }
    }
}

impl App for MyApp {
    fn init(&mut self) {
        self.frame.set_title("MyApp");

        self.frame.add_widget(&self.txt);
    }

    fn on_event(&mut self, _: &Event) -> EventResult {
        EventResult::None
    }
}

fn main() -> Result<(), AppError> {
    AppDelegate::<MyApp>::new("io.github.eza-rs.HelloWorld").run()
}
