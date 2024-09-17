use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub struct Buzzer(web_sys::OscillatorNode);

impl Buzzer {
  pub fn new() -> Buzzer {
        let audio_context = web_sys::AudioContext::new().unwrap();
        let oscillator = audio_context.create_oscillator().unwrap();
        oscillator.set_type(web_sys::OscillatorType::Sine);
        oscillator.connect_with_audio_node(&audio_context.destination()).unwrap();

	Buzzer(oscillator)
  }

  pub fn start(&mut self) -> Result<(), JsValue> {
    self.0.start()?;
    Ok(())
  }

  pub fn set_frequency(&mut self, frequency: f32) -> Result<(), JsValue> {
    self.0.frequency().set_value_at_time(frequency, self.0.context().current_time())?;
    Ok(())
  }
}
