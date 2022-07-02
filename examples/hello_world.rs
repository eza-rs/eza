use eza::{
    widget::{Frame, Label},
	eza_app, AppError
};

#[eza_app("io.github.eza-rs.HelloWorld")]
fn my_app() -> Result<(), AppError>
{
    let mut frame = Frame::new().unwrap();
    let label = Label::new("Hello, world!").unwrap();

	frame.set_title("My App");
	frame.add_widget(&label);

	Ok(())
}
