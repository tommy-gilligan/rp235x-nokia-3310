pub trait Buzzer {
    fn set_frequency(&mut self, frequency: u32);
    fn enable(&mut self);
    fn disable(&mut self);
}

// optional blanket implementation for hertz?
