use web_sys::{
    AudioContext,
    OscillatorType,
    OscillatorNode,
};

pub struct Buzzer(OscillatorNode);

impl Buzzer {
    pub fn new() -> Buzzer {
        let audio_context = AudioContext::new().unwrap();
        let oscillator = audio_context.create_oscillator().unwrap();
        oscillator.set_type(OscillatorType::Sine);
        oscillator.connect_with_audio_node(&audio_context.destination()).unwrap();

        Buzzer(oscillator)
    }
}

impl app::buzzer::Buzzer for Buzzer {
    fn enable(&mut self) {
        self.0.start();
    }

    fn disable(&mut self) {
        self.0.stop();
    }

    fn set_frequency(&mut self, frequency: u32) {
        self.0.frequency().set_value_at_time(
            frequency as f32,
            self.0.context().current_time()
        );
    }
}
