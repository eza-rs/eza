// TODO: Document when events are finished

pub enum KeyCode {
    Space,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub struct MousePosition(i32, i32);

pub enum Event {
    Load,
    Unload,
    KeyUp(KeyCode),
    KeyDown(KeyCode),
    KeyPress(KeyCode),
    MouseUp(MouseButton),
    MouseDown(MouseButton),
    MousePress(MouseButton),
    MouseMove(MousePosition),
}

pub enum EventResult {
    None,
    Skip,
}
