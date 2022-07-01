mod button;
mod frame;
mod label;

pub use button::*;
pub use frame::*;
pub use label::*;

use cacao::view::View;

pub trait CocoaWidget {
    fn add_to_view(&self, view: &View);
}
