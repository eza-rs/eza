pub enum KeyCode {
    Space,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub struct MouseData {
    pub x: i32,
    pub y: i32,
}

pub enum Event {
    KeyUp(KeyCode),
    KeyDown(KeyCode),
    KeyPress(KeyCode),
    MouseUp(MouseButton),
    MouseDown(MouseButton),
    MousePress(MouseButton),
    MouseMove(MouseData),
}

pub enum EventResult {
    None,
    Skip,
}
