mod button;
mod frame;

pub use button::*;
use cacao::view::View;
pub use frame::*;

pub trait OSXWidget {
    fn add_to_view(&self, view: &View);
}
