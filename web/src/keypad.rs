use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};
use wasm_bindgen::closure::Closure;
use multi_tap::Keypad;

mod button;
pub use button::*;

pub struct DomK {
    cancel: Element,
    select: Element,
    updown: Element,
    one: Element,
    two: Element,
    three: Element,
    four: Element,
    five: Element,
    six: Element,
    seven: Element,
    eight: Element,
    nine: Element,
    asterisk: Element,
    zero: Element,
    hash: Element,
    closure: Option<Closure<dyn FnMut(MouseEvent)>>
}

impl DomK {
    pub fn new(
    	cancel: Element,
    	select: Element,
    	updown: Element,
    	one: Element,
    	two: Element,
    	three: Element,
    	four: Element,
    	five: Element,
    	six: Element,
    	seven: Element,
    	eight: Element,
    	nine: Element,
    	asterisk: Element,
    	zero: Element,
    	hash: Element,
    ) -> Self {
        Self {
    	    cancel,
    	    select,
    	    updown,
    	    one,
    	    two,
    	    three,
    	    four,
    	    five,
    	    six,
    	    seven,
    	    eight,
    	    nine,
    	    asterisk,
    	    zero,
    	    hash,
            closure: None
        }
    }

    pub fn init(&mut self) {
        let closure = self.closure.as_ref().unwrap();

        self.cancel.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.select.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.updown.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.one.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.two.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.three.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.four.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.five.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.six.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.seven.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.eight.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.nine.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.zero.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.asterisk.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        self.hash.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    }

    pub fn mouse_event(&self, event: web_sys::MouseEvent) -> Option<multi_tap::keypad::Event<Button>> {
        let binding = event.target().unwrap();
        let target = binding.dyn_ref::<Element>().unwrap();

        if (&self.cancel) == target {
            Some(multi_tap::keypad::Event::Down(Button::Cancel))
        } else if (&self.select) == target {
            Some(multi_tap::keypad::Event::Down(Button::Select))
        } else if (&self.updown) == target {
            Some(multi_tap::keypad::Event::Down(Button::UpDown))
        } else if (&self.one) == target {
            Some(multi_tap::keypad::Event::Down(Button::One))
        } else if (&self.two) == target {
            Some(multi_tap::keypad::Event::Down(Button::Two))
        } else if (&self.three) == target {
            Some(multi_tap::keypad::Event::Down(Button::Three))
        } else if (&self.four) == target {
            Some(multi_tap::keypad::Event::Down(Button::Four))
        } else if (&self.five) == target {
            Some(multi_tap::keypad::Event::Down(Button::Five))
        } else if (&self.six) == target {
            Some(multi_tap::keypad::Event::Down(Button::Six))
        } else if (&self.seven) == target {
            Some(multi_tap::keypad::Event::Down(Button::Seven))
        } else if (&self.eight) == target {
            Some(multi_tap::keypad::Event::Down(Button::Eight))
        } else if (&self.nine) == target {
            Some(multi_tap::keypad::Event::Down(Button::Nine))
        } else if (&self.zero) == target {
            Some(multi_tap::keypad::Event::Down(Button::Zero))
        } else if (&self.asterisk) == target {
            Some(multi_tap::keypad::Event::Down(Button::Asterisk))
        } else if (&self.hash) == target {
            Some(multi_tap::keypad::Event::Down(Button::Hash))
        } else {
            None
        }
    }
}

// impl Keypad for DomK {
//   type Button = Button;
// 
//   async fn event(&mut self) -> multi_tap::keypad::Event<Button> {
//     return multi_tap::keypad::Event::Down(Button::One);
//     // loop {
//     //   Timer::after_millis(1).await;
//     //   if let Some(multi_tap::keypad::Event::Down(e)) = self.last_event.borrow().clone() {
//     //       self.last_event.replace(None);
//     //       return multi_tap::keypad::Event::Down(e);
//     //   }
//     // }
//   }
// }
