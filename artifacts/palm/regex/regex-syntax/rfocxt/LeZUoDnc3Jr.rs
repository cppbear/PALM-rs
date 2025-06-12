use std::cmp::Ordering;
use std::error;
use std::fmt;
pub use ast::visitor::{Visitor, visit};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    /// The span of this class.
    pub span: Span,
    /// Whether this class is negated or not.
    ///
    /// Note: be careful when using this attribute. This specifically refers
    /// to whether the class is written as `\p` or `\P`, where the latter
    /// is `negated = true`. However, it also possible to write something like
    /// `\P{scx!=Katakana}` which is actually equivalent to
    /// `\p{scx=Katakana}` and is therefore not actually negated even though
    /// `negated = true` here. To test whether this class is truly negated
    /// or not, use the `is_negated` method.
    pub negated: bool,
    /// The kind of Unicode class.
    pub kind: ClassUnicodeKind,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassUnicodeKind {
    /// A one letter abbreviated class, e.g., `\pN`.
    OneLetter(char),
    /// A binary property, general category or script. The string may be
    /// empty.
    Named(String),
    /// A property name and an associated value.
    NamedValue {
        /// The type of Unicode op used to associate `name` with `value`.
        op: ClassUnicodeOpKind,
        /// The property name (which may be empty).
        name: String,
        /// The property value (which may be empty).
        value: String,
    },
}
impl ClassUnicode {
    pub fn is_negated(&self) -> bool {
        match self.kind {
            ClassUnicodeKind::NamedValue { op: ClassUnicodeOpKind::NotEqual, .. } => {
                !self.negated
            }
            _ => self.negated,
        }
    }
}
