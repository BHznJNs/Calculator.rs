use std::fmt;

use super::GetAddr;

pub struct Unique(String);

impl Unique {
    pub fn get_identi<'a>(&'a self) -> &'a str {
        return &self.0;
    }
}

impl fmt::Display for Unique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unique({})", self.0)
    }
}

impl From<String> for Unique {
    fn from(value: String) -> Self {
        return Self(value);
    }
}

impl GetAddr for Unique {
    fn get_addr(&self) -> super::Addr {
        let ptr = self as *const Unique;
        return ptr as super::Addr;
    }
}
