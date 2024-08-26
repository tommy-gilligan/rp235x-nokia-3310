#![no_std]
use embassy_futures::select::{select4, Either4};
use embedded_hal_async::digital::Wait;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyPress {
    Select,
    Cancel,
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

pub struct Keypad<
    SelectT,
    CancelT,
    UpT,
    DownT,
    OneT,
    TwoT,
    ThreeT,
    FourT,
    FiveT,
    SixT,
    SevenT,
    EightT,
    NineT,
    AsteriskT,
    ZeroT,
    HashT,
> where
    SelectT: Wait,
    CancelT: Wait,
    UpT: Wait,
    DownT: Wait,
    OneT: Wait,
    TwoT: Wait,
    ThreeT: Wait,
    FourT: Wait,
    FiveT: Wait,
    SixT: Wait,
    SevenT: Wait,
    EightT: Wait,
    NineT: Wait,
    AsteriskT: Wait,
    ZeroT: Wait,
    HashT: Wait,
{
    select: SelectT,
    cancel: CancelT,
    up: UpT,
    down: DownT,
    one: OneT,
    two: TwoT,
    three: ThreeT,
    four: FourT,
    five: FiveT,
    six: SixT,
    seven: SevenT,
    eight: EightT,
    nine: NineT,
    asterisk: AsteriskT,
    zero: ZeroT,
    hash: HashT,
    latch: Option<KeyPress>
}

impl<
        SelectT,
        CancelT,
        UpT,
        DownT,
        OneT,
        TwoT,
        ThreeT,
        FourT,
        FiveT,
        SixT,
        SevenT,
        EightT,
        NineT,
        AsteriskT,
        ZeroT,
        HashT,
    >
    Keypad<
        SelectT,
        CancelT,
        UpT,
        DownT,
        OneT,
        TwoT,
        ThreeT,
        FourT,
        FiveT,
        SixT,
        SevenT,
        EightT,
        NineT,
        AsteriskT,
        ZeroT,
        HashT,
    >
where
    SelectT: Wait,
    CancelT: Wait,
    UpT: Wait,
    DownT: Wait,
    OneT: Wait,
    TwoT: Wait,
    ThreeT: Wait,
    FourT: Wait,
    FiveT: Wait,
    SixT: Wait,
    SevenT: Wait,
    EightT: Wait,
    NineT: Wait,
    AsteriskT: Wait,
    ZeroT: Wait,
    HashT: Wait,
{
    pub fn new(select: SelectT, cancel: CancelT, up: UpT, down: DownT, one: OneT, two: TwoT, three: ThreeT, four: FourT, five: FiveT, six: SixT, seven: SevenT, eight: EightT, nine: NineT, asterisk: AsteriskT, zero: ZeroT, hash: HashT,) -> Self {
	Self {
	    select,
	    cancel,
	    up,
	    down,
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
        latch: None
	}
    }

    // TODO: should debounce in addition to latching
    async fn clear_latch(&mut self) {
        match self.latch {
            Some(KeyPress::Select) => { self.select.wait_for_low().await; },
            Some(KeyPress::Cancel) => { self.cancel.wait_for_low().await; },
            Some(KeyPress::Up) => { self.up.wait_for_low().await; },
            Some(KeyPress::Down) => { self.down.wait_for_low().await; },
            Some(KeyPress::One) => { self.one.wait_for_low().await; },
            Some(KeyPress::Two) => { self.two.wait_for_low().await; },
            Some(KeyPress::Three) => { self.three.wait_for_low().await; },
            Some(KeyPress::Four) => { self.four.wait_for_low().await; },
            Some(KeyPress::Five) => { self.five.wait_for_low().await; },
            Some(KeyPress::Six) => { self.six.wait_for_low().await; },
            Some(KeyPress::Seven) => { self.seven.wait_for_low().await; },
            Some(KeyPress::Eight) => { self.eight.wait_for_low().await; },
            Some(KeyPress::Nine) => { self.nine.wait_for_low().await; },
            Some(KeyPress::Asterisk) => { self.asterisk.wait_for_low().await; },
            Some(KeyPress::Zero) => { self.zero.wait_for_low().await; },
            Some(KeyPress::Hash) => { self.hash.wait_for_low().await; },
            None => ()
        }
    }

    pub async fn key_down(&mut self) -> KeyPress {
        self.clear_latch().await;
        let new_latch_value = match select4(
            select4(
                self.hash.wait_for_high(),
                self.zero.wait_for_high(),
                self.one.wait_for_high(),
                self.two.wait_for_high(),
            ),
            select4(
                self.asterisk.wait_for_high(),
                self.nine.wait_for_high(),
                self.eight.wait_for_high(),
                self.seven.wait_for_high(),
            ),
            select4(
                self.six.wait_for_high(),
                self.five.wait_for_high(),
                self.four.wait_for_high(),
                self.three.wait_for_high(),
            ),
            select4(
                self.select.wait_for_high(),
                self.cancel.wait_for_high(),
                self.up.wait_for_high(),
                self.down.wait_for_high(),
            ),
        )
        .await
        {
            Either4::First(Either4::First(_)) => KeyPress::Hash,
            Either4::First(Either4::Second(_)) => KeyPress::Zero,
            Either4::First(Either4::Third(_)) => KeyPress::One,
            Either4::First(Either4::Fourth(_)) => KeyPress::Two,
            Either4::Second(Either4::First(_)) => KeyPress::Asterisk,
            Either4::Second(Either4::Second(_)) => KeyPress::Nine,
            Either4::Second(Either4::Third(_)) => KeyPress::Eight,
            Either4::Second(Either4::Fourth(_)) => KeyPress::Seven,
            Either4::Third(Either4::First(_)) => KeyPress::Six,
            Either4::Third(Either4::Second(_)) => KeyPress::Five,
            Either4::Third(Either4::Third(_)) => KeyPress::Four,
            Either4::Third(Either4::Fourth(_)) => KeyPress::Three,
            Either4::Fourth(Either4::First(_)) => KeyPress::Select,
            Either4::Fourth(Either4::Second(_)) => KeyPress::Cancel,
            Either4::Fourth(Either4::Third(_)) => KeyPress::Up,
            Either4::Fourth(Either4::Fourth(_)) => KeyPress::Down,
        };
        self.latch = Some(new_latch_value);
        new_latch_value
    }
}

#[cfg(test)]
mod test {
    use super::{KeyPress, Keypad};
    use embedded_hal_mock::eh1::digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction};

    // TODO: improve test. Probably dependent on e-h-m improvements
    #[tokio::test]
    async fn test() {
        let expectations = [];
        let mut select = PinMock::new(&expectations);
        let mut cancel = PinMock::new(&expectations);
        let mut up = PinMock::new(&expectations);
        let mut down = PinMock::new(&expectations);
        let mut one = PinMock::new(&expectations);
        let mut two = PinMock::new(&expectations);
        let mut three = PinMock::new(&expectations);
        let mut four = PinMock::new(&expectations);
        let mut five = PinMock::new(&expectations);
        let mut six = PinMock::new(&expectations);
        let mut seven = PinMock::new(&expectations);
        let mut eight = PinMock::new(&expectations);
        let mut nine = PinMock::new(&expectations);
        let mut asterisk = PinMock::new(&expectations);
        let mut zero = PinMock::new(&expectations);
        let hash_expectations = [PinTransaction::wait_for_state(PinState::High)];
        let mut hash = PinMock::new(&hash_expectations);

        let mut keypad = Keypad {
            select: select.clone(),
            cancel: cancel.clone(),
            up: up.clone(),
            down: down.clone(),
            one: one.clone(),
            two: two.clone(),
            three: three.clone(),
            four: four.clone(),
            five: five.clone(),
            six: six.clone(),
            seven: seven.clone(),
            eight: eight.clone(),
            nine: nine.clone(),
            asterisk: asterisk.clone(),
            zero: zero.clone(),
            hash: hash.clone(),
            latch: None
        };

        assert_eq!(keypad.key_down().await, KeyPress::Hash);

        select.done();
        cancel.done();
        up.done();
        down.done();
        one.done();
        two.done();
        three.done();
        four.done();
        five.done();
        six.done();
        seven.done();
        eight.done();
        nine.done();
        asterisk.done();
        zero.done();
        hash.done();
    }
}
