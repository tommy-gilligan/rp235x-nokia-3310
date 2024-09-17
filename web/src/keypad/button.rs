use core::ascii::Char;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Button {
  Cancel,
  Select,
  UpDown,
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

impl From<Button> for Char {
    fn from(key: Button) -> Char {
        match key {
            Button::One => Char::Digit1,
            Button::Two => Char::CapitalA,
            Button::Three => Char::CapitalD,
            Button::Four => Char::CapitalG,
            Button::Five => Char::CapitalJ,
            Button::Six => Char::CapitalM,
            Button::Seven => Char::CapitalP,
            Button::Eight => Char::CapitalT,
            Button::Nine => Char::CapitalW,
            Button::Asterisk => Char::Asterisk,
            Button::Zero => Char::Space,
            Button::Hash => Char::NumberSign,
            Button::Cancel => Char::Backspace,
            _ => Char::Space,
        }
    }
}
