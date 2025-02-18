use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, Element, GainNode, OscillatorNode, OscillatorType};

pub struct Buzzer {
    element: Element,
    oscillator: Arc<Mutex<Option<OscillatorNode>>>,
    gain: Arc<Mutex<Option<GainNode>>>,
    closure: RefCell<Option<Closure<dyn FnMut()>>>,
}

impl Buzzer {
    pub fn new(element: Element) -> Self {
        let result = Self {
            element,
            oscillator: Arc::new(Mutex::new(None)),
            gain: Arc::new(Mutex::new(None)),
            closure: RefCell::new(None),
        };

        let o = Arc::clone(&result.oscillator);
        let g = Arc::clone(&result.gain);

        result
            .closure
            .borrow_mut()
            .replace(Closure::<dyn FnMut()>::new(move || {
                let mut ox = o.lock().unwrap();
                if ox.is_none() {
                    let audio_context = AudioContext::new().unwrap();
                    let oscillator = audio_context.create_oscillator().unwrap();
                    let gain = audio_context.create_gain().unwrap();

                    oscillator.set_type(OscillatorType::Sine);
                    oscillator.connect_with_audio_node(&gain).unwrap();
                    gain.connect_with_audio_node(&audio_context.destination())
                        .unwrap();
                    ox.replace(oscillator);
                    g.lock().unwrap().replace(gain);
                }
            }));

        result
            .element
            .add_event_listener_with_callback(
                "click",
                result
                    .closure
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();

        result
    }
}

impl shared::Buzzer for Buzzer {
    fn mute(&mut self) {
        let binding = Arc::clone(&self.oscillator);
        let mut oscillator = binding.lock().unwrap();
        if let Some(o) = oscillator.as_mut() {
            match o.stop() {
                Ok(()) => {}
                Err(e) => {
                    let dom_exception: Option<&web_sys::DomException> = e.as_ref().dyn_ref();
                    if dom_exception.unwrap().code() != web_sys::DomException::INVALID_STATE_ERR {
                        // panic!("HEY");
                    }
                }
            }
        }
    }

    fn unmute(&mut self) {
        let binding = Arc::clone(&self.oscillator);
        let mut oscillator = binding.lock().unwrap();
        if let Some(o) = oscillator.as_mut() {
            match o.start() {
                Ok(()) => {}
                Err(e) => {
                    let dom_exception: Option<&web_sys::DomException> = e.as_ref().dyn_ref();
                    if dom_exception.unwrap().code() != web_sys::DomException::INVALID_STATE_ERR {
                        // panic!("HEY");
                    }
                }
            }
        }
    }

    fn set_frequency(&mut self, frequency: u16) {
        let binding = Arc::clone(&self.oscillator);
        let mut oscillator = binding.lock().unwrap();
        if let Some(o) = oscillator.as_mut() {
            o.frequency().set_value(frequency as f32);
        }
    }

    fn set_volume(&mut self, volume: u8) {
        let binding = Arc::clone(&self.gain);
        let mut gain = binding.lock().unwrap();
        if let Some(g) = gain.as_mut() {
            g.gain().set_value(volume as f32 / 100.0);
        }
    }
}
