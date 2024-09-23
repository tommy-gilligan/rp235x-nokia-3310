use core::cell::RefCell;
use std::rc::Rc;

use app::keypad::{Button, Keypad};
use embassy_time::Timer;
use wasm_bindgen::{closure::Closure, JsCast};

struct DomB {
    was_clicked: bool,
}

impl DomB {
    fn new(id: &'static str) -> Rc<RefCell<Self>> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let s = Self { was_clicked: false };
        let r = Rc::new(RefCell::new(s));
        let g = r.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            (*g).borrow_mut().was_clicked = true;
        });

        document
            .get_element_by_id(id)
            .unwrap()
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        r
    }

    fn check(&mut self) -> bool {
        let result = self.was_clicked;
        self.was_clicked = false;
        result
    }
}

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
    async fn event(&mut self) -> app::keypad::Event<Button> {
        loop {
            Timer::after_millis(30).await;
            if (*self.cancel).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Cancel);
            } else if (*self.select).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Select);
            } else if (*self.up).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Up);
            } else if (*self.down).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Down);
            } else if (*self.one).borrow_mut().check() {
                return app::keypad::Event::Down(Button::One);
            } else if (*self.two).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Two);
            } else if (*self.three).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Three);
            } else if (*self.four).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Four);
            } else if (*self.five).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Five);
            } else if (*self.six).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Six);
            } else if (*self.seven).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Seven);
            } else if (*self.eight).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Eight);
            } else if (*self.nine).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Nine);
            } else if (*self.asterisk).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Asterisk);
            } else if (*self.zero).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Zero);
            } else if (*self.hash).borrow_mut().check() {
                return app::keypad::Event::Down(Button::Hash);
            }
        }
    }
}
