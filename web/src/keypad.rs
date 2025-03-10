use core::cell::RefCell;
use std::rc::Rc;

use embassy_time::Timer;
use shared::{Key, Keypad};

use super::DomB;

pub struct DomKeypad {
    cancel: Rc<RefCell<DomB>>,
    select: Rc<RefCell<DomB>>,
    up: Rc<RefCell<DomB>>,
    down: Rc<RefCell<DomB>>,
    one: Rc<RefCell<DomB>>,
    two: Rc<RefCell<DomB>>,
    three: Rc<RefCell<DomB>>,
    four: Rc<RefCell<DomB>>,
    five: Rc<RefCell<DomB>>,
    six: Rc<RefCell<DomB>>,
    seven: Rc<RefCell<DomB>>,
    eight: Rc<RefCell<DomB>>,
    nine: Rc<RefCell<DomB>>,
    asterisk: Rc<RefCell<DomB>>,
    zero: Rc<RefCell<DomB>>,
    hash: Rc<RefCell<DomB>>,
}

impl DomKeypad {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cancel_id: &'static str,
        select_id: &'static str,
        up_id: &'static str,
        down_id: &'static str,
        one_id: &'static str,
        two_id: &'static str,
        three_id: &'static str,
        four_id: &'static str,
        five_id: &'static str,
        six_id: &'static str,
        seven_id: &'static str,
        eight_id: &'static str,
        nine_id: &'static str,
        asterisk_id: &'static str,
        zero_id: &'static str,
        hash_id: &'static str,
    ) -> Self {
        Self {
            cancel: DomB::new(cancel_id),
            select: DomB::new(select_id),
            up: DomB::new(up_id),
            down: DomB::new(down_id),
            one: DomB::new(one_id),
            two: DomB::new(two_id),
            three: DomB::new(three_id),
            four: DomB::new(four_id),
            five: DomB::new(five_id),
            six: DomB::new(six_id),
            seven: DomB::new(seven_id),
            eight: DomB::new(eight_id),
            nine: DomB::new(nine_id),
            asterisk: DomB::new(asterisk_id),
            zero: DomB::new(zero_id),
            hash: DomB::new(hash_id),
        }
    }
}

unsafe impl Send for DomKeypad {}

impl Keypad for DomKeypad {
    async fn event(&mut self) -> shared::KeyEvent {
        loop {
            Timer::after_millis(30).await;
            if (*self.cancel).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Cancel);
            } else if (*self.select).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Select);
            } else if (*self.up).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Up);
            } else if (*self.down).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Down);
            } else if (*self.one).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::One);
            } else if (*self.two).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Two);
            } else if (*self.three).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Three);
            } else if (*self.four).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Four);
            } else if (*self.five).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Five);
            } else if (*self.six).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Six);
            } else if (*self.seven).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Seven);
            } else if (*self.eight).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Eight);
            } else if (*self.nine).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Nine);
            } else if (*self.asterisk).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Asterisk);
            } else if (*self.zero).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Zero);
            } else if (*self.hash).borrow_mut().check() {
                return shared::KeyEvent::Down(Key::Hash);
            }
        }
    }
}
