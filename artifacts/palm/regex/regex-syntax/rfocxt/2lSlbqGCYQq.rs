use std::char;
use std::cmp;
use std::error;
use std::fmt;
use std::u8;
use ast::Span;
use hir::interval::{Interval, IntervalSet, IntervalSetIter};
use unicode;
pub use hir::visitor::{Visitor, visit};
macro_rules! define_bool {
    ($bit:expr, $is_fn_name:ident, $set_fn_name:ident) => {
        fn $is_fn_name (& self) -> bool { self.bools & (0b1 << $bit) > 0 } fn
        $set_fn_name (& mut self, yes : bool) { if yes { self.bools |= 1 << $bit; } else
        { self.bools &= ! (1 << $bit); } }
    };
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the translator's Ast was parsed from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error, derived from the Ast given to the translator.
    span: Span,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// This error occurs when a Unicode feature is used when Unicode
    /// support is disabled. For example `(?-u:\pL)` would trigger this error.
    UnicodeNotAllowed,
    /// This error occurs when translating a pattern that could match a byte
    /// sequence that isn't UTF-8 and `allow_invalid_utf8` was disabled.
    InvalidUtf8,
    /// This occurs when an unrecognized Unicode property name could not
    /// be found.
    UnicodePropertyNotFound,
    /// This occurs when an unrecognized Unicode property value could not
    /// be found.
    UnicodePropertyValueNotFound,
    /// This occurs when the translator attempts to construct a character class
    /// that is empty.
    ///
    /// Note that this restriction in the translator may be removed in the
    /// future.
    EmptyClassNotAllowed,
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// An error that occurred while translating concrete syntax into abstract
    /// syntax (AST).
    Parse(ast::Error),
    /// An error that occurred while translating abstract syntax into a high
    /// level intermediate representation (HIR).
    Translate(hir::Error),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// The capturing group limit was exceeded.
    ///
    /// Note that this represents a limit on the total number of capturing
    /// groups in a regex and not necessarily the number of nested capturing
    /// groups. That is, the nest limit can be low and it is still possible for
    /// this error to occur.
    CaptureLimitExceeded,
    /// An invalid escape sequence was found in a character class set.
    ClassEscapeInvalid,
    /// An invalid character class range was found. An invalid range is any
    /// range where the start is greater than the end.
    ClassRangeInvalid,
    /// An invalid range boundary was found in a character class. Range
    /// boundaries must be a single literal codepoint, but this error indicates
    /// that something else was found, such as a nested class.
    ClassRangeLiteral,
    /// An opening `[` was found with no corresponding closing `]`.
    ClassUnclosed,
    /// An empty decimal number was given where one was expected.
    DecimalEmpty,
    /// An invalid decimal number was given where one was expected.
    DecimalInvalid,
    /// A bracketed hex literal was empty.
    EscapeHexEmpty,
    /// A bracketed hex literal did not correspond to a Unicode scalar value.
    EscapeHexInvalid,
    /// An invalid hexadecimal digit was found.
    EscapeHexInvalidDigit,
    /// EOF was found before an escape sequence was completed.
    EscapeUnexpectedEof,
    /// An unrecognized escape sequence.
    EscapeUnrecognized,
    /// A dangling negation was used when setting flags, e.g., `i-`.
    FlagDanglingNegation,
    /// A flag was used twice, e.g., `i-i`.
    FlagDuplicate {
        /// The position of the original flag. The error position
        /// points to the duplicate flag.
        original: Span,
    },
    /// The negation operator was used twice, e.g., `-i-s`.
    FlagRepeatedNegation {
        /// The position of the original negation operator. The error position
        /// points to the duplicate negation operator.
        original: Span,
    },
    /// Expected a flag but got EOF, e.g., `(?`.
    FlagUnexpectedEof,
    /// Unrecognized flag, e.g., `a`.
    FlagUnrecognized,
    /// A duplicate capture name was found.
    GroupNameDuplicate {
        /// The position of the initial occurrence of the capture name. The
        /// error position itself points to the duplicate occurrence.
        original: Span,
    },
    /// A capture group name is empty, e.g., `(?P<>abc)`.
    GroupNameEmpty,
    /// An invalid character was seen for a capture group name. This includes
    /// errors where the first character is a digit (even though subsequent
    /// characters are allowed to be digits).
    GroupNameInvalid,
    /// A closing `>` could not be found for a capture group name.
    GroupNameUnexpectedEof,
    /// An unclosed group, e.g., `(ab`.
    ///
    /// The span of this error corresponds to the unclosed parenthesis.
    GroupUnclosed,
    /// An unopened group, e.g., `ab)`.
    GroupUnopened,
    /// The nest limit was exceeded. The limit stored here is the limit
    /// configured in the parser.
    NestLimitExceeded(u32),
    /// The range provided in a counted repetition operator is invalid. The
    /// range is invalid if the start is greater than the end.
    RepetitionCountInvalid,
    /// An opening `{` was found with no corresponding closing `}`.
    RepetitionCountUnclosed,
    /// A repetition operator was applied to a missing sub-expression. This
    /// occurs, for example, in the regex consisting of just a `*` or even
    /// `(?i)*`. It is, however, possible to create a repetition operating on
    /// an empty sub-expression. For example, `()*` is still considered valid.
    RepetitionMissing,
    /// When octal support is disabled, this error is produced when an octal
    /// escape is used. The octal escape is assumed to be an invocation of
    /// a backreference, which is the common case.
    UnsupportedBackreference,
    /// When syntax similar to PCRE's look-around is used, this error is
    /// returned. Some example syntaxes that are rejected include, but are
    /// not necessarily limited to, `(?=re)`, `(?!re)`, `(?<=re)` and
    /// `(?<!re)`. Note that all of these syntaxes are otherwise invalid; this
    /// error is used to improve the user experience.
    UnsupportedLookAround,
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
    pub fn pattern(&self) -> &str {}
    pub fn span(&self) -> &Span {}
}
