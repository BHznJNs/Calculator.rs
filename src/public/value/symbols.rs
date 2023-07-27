use std::fmt;

use crate::public::error::{internal_error, InternalComponent};

#[derive(PartialEq, Clone, Copy)]
pub enum Symbols {
    // math symbols
    Plus, // low  priority
    Minus,
    Multiply,
    Divide,
    Power, // high priority

    Not,
    LessThan,
    MoreThan,
    LessThanEqual,
    MoreThanEqual,
    CompareEqual,
    NotEqual,

    AndSign,
    OrSign,

    Equal,
    PlusEqual,
    MinusEqual,
    MultiplyEqual,
    DivideEqual,
    PowerEqual,

    ObjectReading,
}

impl Symbols {
    pub fn combine(&self, other: Symbols) -> Result<Self, ()> {
        // example:
        //    let equal_symbol = Symbols::Equal;
        //    equal_symbol.combine(Symbols::Plus);
        //    -> Symbols::PlusEqual

        // only `Symbole::Equal` can call the internal function `Symbols::combine`
        if *self != Self::Equal {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Symbols::combine` invocation",
            )?);
        }

        let result_symbol = match other {
            Self::Plus => Self::PlusEqual,
            Self::Minus => Self::MinusEqual,
            Self::Multiply => Self::MultiplyEqual,
            Self::Divide => Self::DivideEqual,
            Self::Power => Self::PowerEqual,
            Self::LessThan => Self::LessThanEqual,
            Self::MoreThan => Self::MoreThanEqual,
            Self::Not => Self::NotEqual,
            Self::Equal => Self::CompareEqual,
            _ => {
                let msg = format!("invalid symbol `{}` for symbol combination", other);
                return Err(internal_error(InternalComponent::Tokenizer, &msg)?);
            }
        };
        Ok(result_symbol)
    }
    pub fn separate(self) -> Self {
        match self {
            Self::PlusEqual => Self::Plus,
            Self::MinusEqual => Self::Minus,
            Self::MultiplyEqual => Self::Multiply,
            Self::DivideEqual => Self::Divide,
            Self::PowerEqual => Self::Power,
            _ => self,
        }
    }

    pub fn is_basic_symbol(symbol: Self) -> bool {
        return symbol == Self::Plus
            || symbol == Self::Minus
            || symbol == Self::Multiply
            || symbol == Self::Divide
            || symbol == Self::Power
            || symbol == Self::Not
            || symbol == Self::LessThan
            || symbol == Self::MoreThan
            || symbol == Self::Equal;
    }

    pub fn is_equal_symbol(symbol: Self) -> bool {
        return symbol == Self::Equal
            || symbol == Self::PlusEqual
            || symbol == Self::MinusEqual
            || symbol == Self::MultiplyEqual
            || symbol == Self::DivideEqual
            || symbol == Self::PowerEqual;
    }
}

impl From<char> for Symbols {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            '^' => Self::Power,
            '!' => Self::Not,
            '&' => Self::AndSign,
            '|' => Self::OrSign,
            '<' => Self::LessThan,
            '>' => Self::MoreThan,
            '=' => Self::Equal,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Multiply => write!(f, "Multiply"),
            Self::Divide => write!(f, "Divide"),
            Self::Power => write!(f, "Power"),

            Self::Not => write!(f, "Not"),
            Self::AndSign => write!(f, "And"),
            Self::OrSign => write!(f, "Or"),

            Self::LessThan => write!(f, "LessThan"),
            Self::MoreThan => write!(f, "MoreThan"),
            Self::Equal => write!(f, "Equal"),
            Self::LessThanEqual => write!(f, "LessThanEqual"),
            Self::MoreThanEqual => write!(f, "MoreThanEqual"),
            Self::NotEqual => write!(f, "NotEqual"),
            Self::CompareEqual => write!(f, "CompareEqual"),

            Self::PlusEqual => write!(f, "PlusEqual"),
            Self::MinusEqual => write!(f, "MinusEqual"),
            Self::MultiplyEqual => write!(f, "MultiplyEqual"),
            Self::DivideEqual => write!(f, "DivideEqual"),
            Self::PowerEqual => write!(f, "PowerEqual"),

            Self::ObjectReading => write!(f, "ObjectReading"),
        }
    }
}
