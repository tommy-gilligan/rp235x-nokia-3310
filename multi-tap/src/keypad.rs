use core::{
    ascii::Char,
    future::Future
};

use defmt::Format;

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Event<B> {
    Down(B),
    Up(B),
}

pub trait Keypad {
    type Button: Into<Char> + PartialEq + Clone;

    fn event(&mut self) -> impl Future<Output = Event<Self::Button>> + Send;
}
