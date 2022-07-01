mod button;
mod frame;

pub use button::*;
pub use frame::*;

use cacao::view::View;

pub trait CocoaWidget {
    fn add_to_view(&self, view: &View);
}
