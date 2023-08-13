use std::{fmt, rc::Rc};

use super::GetAddr;

#[derive(Clone)]
pub struct Unique(Rc<String>);

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

impl From<&str> for Unique {
    fn from(value: &str) -> Self {
        return Self(String::from(value).into());
    }
}
impl From<String> for Unique {
    fn from(value: String) -> Self {
        return Self(value.into());
    }
}

impl GetAddr for Unique {
    fn get_addr(&self) -> super::Addr {
        let ptr = self as *const Unique;
        return ptr as super::Addr;
    }
}

// --- --- --- --- --- ---

pub struct GlobalUnique {
    pub value: Option<Unique>,
}
impl GlobalUnique {
    pub fn init(&mut self, identi: &str) {
        self.value = Some(Unique::from(identi));
    }

    pub fn unwrap(&self) -> Unique {
        let value_ref = self.value.as_ref();
        let Some(wraped) = value_ref else {
            unreachable!()
        };
        return wraped.clone();
    }
}
