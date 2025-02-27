use js_sys::Date;
use shared::Rtc;

pub struct Clock {
    offset: i64,
}

impl Clock {
    pub fn new() -> Self {
        Self { offset: 0 }
    }

    // fn set_timestamp(&mut self, timestamp: i64) {
    //     self.offset = timestamp - (Date::now() / 1000.0) as i64;
    // }
}

impl Rtc for Clock {
    fn timestamp(&mut self) -> i64 {
        (Date::now() / 1000.0) as i64 + self.offset
    }
}
