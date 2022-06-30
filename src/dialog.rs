pub enum DialogResult {
    Ok,
    Error,
}

pub struct DialogBuilder {
    title: &'static str,
    message: &'static str,
}

impl DialogBuilder {
    pub fn new() -> Self {
        Self {
            title: "",
            message: "",
        }
    }

    pub fn title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn message(&mut self, message: &'static str) -> &mut Self {
        self.message = message;
        self
    }

    pub fn show(&mut self) -> DialogResult {
        DialogResult::Ok
    }
}
