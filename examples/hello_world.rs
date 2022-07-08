use eza::{
    dialog::DialogBuilder,
    widget::{Button, Frame},
    App, AppError,
};

fn main() -> Result<(), AppError> {
    let app = App::new("io.github.eza-rs.HelloWorld")?;
	
	let mut frame = Frame::new()?;

    frame.set_title(file!());

    let mut button = Button::new("Hello, world!")?;
	button.set_action(|| println!("You pressed the button!"));

    frame.add_widget(button);

    DialogBuilder::new()
        .title("Message")
        .message("Hello, world!")
        .show();

	app.run()
}
