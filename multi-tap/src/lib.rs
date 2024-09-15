#![no_std]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(trivial_bounds)]
#![feature(let_chains)]

pub mod char;
pub mod keypad;

use core::{ascii::Char, future::Future};

use defmt::Format;
use futures::{future, future::Either, pin_mut};
pub use keypad::*;

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Event {
    Tentative(Char),
    Decided(Char),
}

pub struct MultiTap<KEYPAD>
where
    KEYPAD: Keypad,
{
    keypad: KEYPAD,
    last_press: Option<KEYPAD::Button>,
    last_emitted: Option<Event>,
    pending: Option<Event>,
}

impl<KEYPAD> MultiTap<KEYPAD>
where
    KEYPAD: Keypad,
{
    pub fn new(keypad: KEYPAD) -> Self {
        Self {
            keypad,
            last_press: None,
            last_emitted: None,
            pending: None,
        }
    }

    pub async fn event<T>(&mut self, timeout_future: T) -> Event
    where
        T: Future<Output = ()>,
    {
        // if something has just been decided
        // still emit the next tentative
        if let Some(pending) = self.pending {
            self.pending = None;
            self.last_emitted = Some(pending.clone());
            return pending;
        }

        let event_future = self.keypad.event();
        pin_mut!(event_future);
        pin_mut!(timeout_future);

        if self.last_press.is_some() {
            match future::select(timeout_future, event_future).await {
                Either::Left((_, _)) => {
                    if let Some(Event::Tentative(e)) = self.last_emitted {
                        self.last_emitted = None;
                        self.last_press = None;

                        return Event::Decided(e);
                    }
                }
                Either::Right((keypad::Event::Down(e), _)) => {
                    if let Some(p) = &self.last_press
                        && *p != e
                    {
                        if let Some(Event::Tentative(f)) = self.last_emitted {
                            self.last_emitted = Some(Event::Tentative(e.clone().into()));
                            self.last_press = Some(e.clone());
                            // TODO: panic if there is already a pending event
                            self.pending = Some(Event::Tentative(e.clone().into()));

                            return Event::Decided(f);
                        }
                    } else {
                        self.last_press = Some(e.clone());
                        self.last_emitted = match self.last_emitted {
                            Some(Event::Tentative(c)) => Some(Event::Tentative(char::next_char(c))),
                            Some(Event::Decided(_)) => None,
                            None => Some(Event::Tentative(e.clone().into())),
                        };

                        return self.last_emitted.unwrap();
                    }
                }
                Either::Right((keypad::Event::Up(_), _)) => {}
            }
        } else {
            if let keypad::Event::Down(e) = event_future.await {
                if let Some(p) = &self.last_press
                    && *p != e
                {
                    if let Some(Event::Tentative(f)) = self.last_emitted {
                        self.last_emitted = Some(Event::Tentative(e.clone().into()));
                        self.last_press = Some(e.clone());
                        // TODO: panic if there is already a pending event
                        self.pending = Some(Event::Tentative(e.clone().into()));

                        return Event::Decided(f);
                    }
                } else {
                    self.last_press = Some(e.clone());
                    self.last_emitted = match self.last_emitted {
                        Some(Event::Tentative(c)) => Some(Event::Tentative(char::next_char(c))),
                        Some(Event::Decided(_)) => None,
                        None => Some(Event::Tentative(e.clone().into())),
                    };

                    return self.last_emitted.unwrap();
                }
            }
        }

        panic!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::time::Duration;
    use tokio::time::sleep;

    #[derive(Debug, PartialEq, Format, Copy, Clone)]
    pub enum Key {
        One,
        Two,
    }

    impl From<Key> for Char {
        fn from(key: Key) -> Char {
            match key {
                Key::One => Char::Digit1,
                Key::Two => Char::CapitalA,
            }
        }
    }

    struct TwoKeys<'a>(&'a [Key], usize);

    impl<'a> TwoKeys<'a> {
        fn new(presses: &'a [Key]) -> Self {
            TwoKeys(presses, 0)
        }
    }

    impl Keypad for TwoKeys<'_> {
        type Button = Key;

        async fn event(&mut self) -> crate::keypad::Event<Self::Button> {
            let result = self.0[self.1];
            self.1 += 1;
            crate::keypad::Event::Down(result)
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn test_timeout() {
        let presses = [];
        let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
        multi_tap.event(async {}).await;
    }

    #[tokio::test]
    async fn test_one() {
        let presses = [Key::One];
        let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit1)
        )
    }

    #[tokio::test]
    async fn test_one_two() {
        let presses = [Key::One, Key::Two];
        let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));

        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit1)
        );
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Decided(Char::Digit1)
        );
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::CapitalA)
        );
    }

    #[tokio::test]
    async fn test_one_one() {
        let presses = [Key::One, Key::One];
        let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));

        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit1)
        );
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit2)
        );
    }

    #[tokio::test]
    async fn test_one_timeout() {
        let presses = [Key::One];
        let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit1)
        );
        assert_eq!(
            multi_tap.event(async {}).await,
            Event::Decided(Char::Digit1)
        );
    }

    #[tokio::test]
    async fn test_one_two_timeout() {
        let presses = [Key::One, Key::Two];
        let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));

        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit1)
        );
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Decided(Char::Digit1)
        );
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::CapitalA)
        );
        assert_eq!(
            multi_tap.event(async {}).await,
            Event::Decided(Char::CapitalA)
        );
    }

    #[tokio::test]
    async fn test_one_one_timeout() {
        let presses = [Key::One, Key::One];
        let mut multi_tap = MultiTap::new(TwoKeys::new(&presses));

        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit1)
        );
        assert_eq!(
            multi_tap.event(sleep(Duration::from_secs(100))).await,
            Event::Tentative(Char::Digit2)
        );
        assert_eq!(
            multi_tap.event(async {}).await,
            Event::Decided(Char::Digit2)
        );
    }
}
