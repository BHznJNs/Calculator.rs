pub mod value;

pub mod array;
pub mod function;
pub mod number;
pub mod symbols;

pub mod oop;

#[inline]
fn display_indent(level: usize) -> String {
    "  ".repeat(level)
}
