use strum::Display;
use yew::html::ImplicitClone;

#[derive(Clone, Copy, Display, PartialEq)]
pub enum Player {
    None,
    Red,
    Blue
}

impl ImplicitClone for Player {}
