use std::cmp::Ordering;
use std::error;
use std::fmt;
pub use ast::visitor::{Visitor, visit};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlagsItem {
    /// The span of this item.
    pub span: Span,
    /// The kind of this item.
    pub kind: FlagsItemKind,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlagsItemKind {
    /// A negation operator applied to all subsequent flags in the enclosing
    /// group.
    Negation,
    /// A single flag in a group.
    Flag(Flag),
}
impl Flags {
    pub fn add_item(&mut self, item: FlagsItem) -> Option<usize> {
        for (i, x) in self.items.iter().enumerate() {
            if x.kind == item.kind {
                return Some(i);
            }
        }
        self.items.push(item);
        None
    }
    pub fn flag_state(&self, flag: Flag) -> Option<bool> {}
}
