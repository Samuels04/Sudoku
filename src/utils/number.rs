pub enum Number {
    EMPTY = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGTH = 8,
    NINE = 9,
    ERROR,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Number {
    pub fn parse(s: String) -> Number {
        let number: i8 = s.trim().parse().unwrap();
        match number {
            0 => return Number::EMPTY,
            1 => return Number::ONE,
            2 => return Number::TWO,
            3 => return Number::THREE,
            4 => return Number::FOUR,
            5 => return Number::FIVE,
            6 => return Number::SIX,
            7 => return Number::SEVEN,
            8 => return Number::EIGTH,
            9 => return Number::NINE,
            _ => return Number::ERROR,
        }
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Self::EMPTY => Self::EMPTY,
            Self::ONE => Self::ONE,
            Self::TWO => Self::TWO,
            Self::THREE => Self::THREE,
            Self::FOUR => Self::FOUR,
            Self::FIVE => Self::FIVE,
            Self::SIX => Self::SIX,
            Self::SEVEN => Self::SEVEN,
            Self::EIGTH => Self::EIGTH,
            Self::NINE => Self::NINE,
            Self::ERROR => Self::ERROR,
        }
    }
}

impl Copy for Number {

}
