use eza::{
    dialog::DialogBuilder,
    widget::{Button, Frame},
    AppError,
};

#[eza::eza_app("io.github.eza-rs.HelloWorld")]
fn main() -> Result<(), AppError> {
    let mut frame = Frame::new()?;

    frame.set_title(file!());

    let button = Button::new("Hello, world!")?;

    frame.add_widget(button);

    DialogBuilder::new()
        .title("Message")
        .message("Hello, world!")
        .show();

    Ok(())
}
