use std::cmp::Ordering;
use std::error;
use std::fmt;
pub use ast::visitor::{Visitor, visit};
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlagsItemKind {
    /// A negation operator applied to all subsequent flags in the enclosing
    /// group.
    Negation,
    /// A single flag in a group.
    Flag(Flag),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Flag {
    /// `i`
    CaseInsensitive,
    /// `m`
    MultiLine,
    /// `s`
    DotMatchesNewLine,
    /// `U`
    SwapGreed,
    /// `u`
    Unicode,
    /// `x`
    IgnoreWhitespace,
}
impl FlagsItemKind {
    pub fn is_negation(&self) -> bool {
        match *self {
            FlagsItemKind::Negation => true,
            _ => false,
        }
    }
}
