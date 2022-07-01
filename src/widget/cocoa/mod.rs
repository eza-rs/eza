mod button;
mod frame;
mod label;
mod textbox;

pub use button::*;
pub use frame::*;
pub use label::*;
pub use textbox::*;

use cacao::view::View;

pub trait CocoaWidget {
    fn add_to_view(&self, view: &View);
}
