use std::cmp::Ordering;
use std::error;
use std::fmt;
pub use ast::visitor::{Visitor, visit};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    /// The span of this group.
    pub span: Span,
    /// The kind of this group.
    pub kind: GroupKind,
    /// The regular expression in this group.
    pub ast: Box<Ast>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ast {
    /// An empty regex that matches everything.
    Empty(Span),
    /// A set of flags, e.g., `(?is)`.
    Flags(SetFlags),
    /// A single character literal, which includes escape sequences.
    Literal(Literal),
    /// The "any character" class.
    Dot(Span),
    /// A single zero-width assertion.
    Assertion(Assertion),
    /// A single character class. This includes all forms of character classes
    /// except for `.`. e.g., `\d`, `\pN`, `[a-z]` and `[[:alpha:]]`.
    Class(Class),
    /// A repetition operator applied to an arbitrary regular expression.
    Repetition(Repetition),
    /// A grouped regular expression.
    Group(Group),
    /// An alternation of regular expressions.
    Alternation(Alternation),
    /// A concatenation of regular expressions.
    Concat(Concat),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupKind {
    /// `(a)`
    CaptureIndex(u32),
    /// `(?P<name>a)`
    CaptureName(CaptureName),
    /// `(?:a)` and `(?i:a)`
    NonCapturing(Flags),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupKind {
    /// A normal unnamed capturing group.
    ///
    /// The value is the capture index of the group.
    CaptureIndex(u32),
    /// A named capturing group.
    CaptureName {
        /// The name of the group.
        name: String,
        /// The capture index of the group.
        index: u32,
    },
    /// A non-capturing group.
    NonCapturing,
}
impl Group {
    pub fn flags(&self) -> Option<&Flags> {}
    pub fn is_capturing(&self) -> bool {
        match self.kind {
            GroupKind::CaptureIndex(_) | GroupKind::CaptureName(_) => true,
            GroupKind::NonCapturing(_) => false,
        }
    }
    pub fn capture_index(&self) -> Option<u32> {}
}
