use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Symbols {
    // math symbols
    Plus,  // low  priority
    Minus,
    Multiply,
    Divide,
    Power, // high priority

    LessThan,
    MoreThan,
    LessThanEqual,
    MoreThanEqual,
    CompareEqual,
    Equal,

    PlusEqual,
    MinusEqual,
    MultiplyEqual,
    DivideEqual,
    PowerEqual,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl Symbols {
    pub fn combine(&self, other: Symbols) -> Result<Symbols, ()> {
        // example:
        //    let equal_symbol = Symbols::Equal;
        //    equal_symbol.combine(Symbols::Plus);
        //    -> Symbols::PlusEqual

        if *self != Symbols::Equal {
            println!("Only Symbole::Equal can call the Symbols::combine.");
            return Err(())
        }

        let result_symbol =
        match other {
            Symbols::Plus     => Symbols::PlusEqual,
            Symbols::Minus    => Symbols::MinusEqual,
            Symbols::Multiply => Symbols::MultiplyEqual,
            Symbols::Divide   => Symbols::DivideEqual,
            Symbols::Power    => Symbols::PowerEqual,
            Symbols::LessThan => Symbols::LessThanEqual,
            Symbols::MoreThan => Symbols::MoreThanEqual,
            Symbols::Equal    => Symbols::CompareEqual,
            _ => {
                println!("Invalid symbol: {}", other);
                return Err(())
            }
        };
        Ok(result_symbol)
    }
    pub fn separate(self) -> Symbols {
        match self {
            Symbols::PlusEqual => Symbols::Plus,
            Symbols::MinusEqual => Symbols::Minus,
            Symbols::MultiplyEqual => Symbols::Multiply,
            Symbols::DivideEqual => Symbols::Divide,
            Symbols::PowerEqual => Symbols::Power,
            _ => self
        }
    }

    pub fn is_basic_symbol(symbol: Symbols) -> bool {
        return symbol == Symbols::Plus  ||
               symbol == Symbols::Minus ||
               symbol == Symbols::Multiply ||
               symbol == Symbols::Divide ||
               symbol == Symbols::Power  ||
               symbol == Symbols::LessThan ||
               symbol == Symbols::MoreThan ||
               symbol == Symbols::Equal
    }

    pub fn is_equal_symbol(symbol: Symbols) -> bool {
        return symbol == Symbols::Equal ||
               symbol == Symbols::PlusEqual ||
               symbol == Symbols::MinusEqual ||
               symbol == Symbols::MultiplyEqual ||
               symbol == Symbols::DivideEqual ||
               symbol == Symbols::PowerEqual
    }
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbols::Plus     => write!(f, "Plus"),
            Symbols::Minus    => write!(f, "Minus"),
            Symbols::Multiply => write!(f, "Multiply"),
            Symbols::Divide   => write!(f, "Divide"),
            Symbols::Power    => write!(f, "Power"),
            Symbols::LessThan => write!(f, "LessThan"),
            Symbols::MoreThan => write!(f, "MoreThan"),
            Symbols::LessThanEqual => write!(f, "LessThanEqual"),
            Symbols::MoreThanEqual => write!(f, "MoreThanEqual"),
            Symbols::CompareEqual  => write!(f, "CompareEqual"),
            Symbols::Equal    => write!(f, "Equal"),

            Symbols::PlusEqual     => write!(f, "PlusEqual"),
            Symbols::MinusEqual    => write!(f, "MinusEqual"),
            Symbols::MultiplyEqual => write!(f, "MultiplyEqual"),
            Symbols::DivideEqual   => write!(f, "DivideEqual"),
            Symbols::PowerEqual    => write!(f, "PowerEqual"),
        
            Symbols::LeftParen  => write!(f, "LeftParen"),
            Symbols::RightParen => write!(f, "RightParen"),
            Symbols::LeftBrace  => write!(f, "LeftBrace"),
            Symbols::RightBrace => write!(f, "RightBrace"),
        }
    }
}