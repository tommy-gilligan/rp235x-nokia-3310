use core::future::Future;

#[derive(PartialEq, Clone, Copy)]
pub enum Button {
    Cancel,
    Select,
    Up,
    Down,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Asterisk,
    Zero,
    Hash,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Event<B> {
    Down(B),
    Up(B),
}

pub trait Keypad {
    fn event(&mut self) -> impl Future<Output = Event<Button>> + Send;
}
