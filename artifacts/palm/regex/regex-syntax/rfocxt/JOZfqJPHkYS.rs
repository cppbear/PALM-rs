type Result<T> = result::Result<T, ast::Error>;
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::mem;
use std::result;
use ast::{self, Ast, Position, Span};
use either::Either;
use is_meta_character;
#[derive(Debug)]
struct NestLimiter<'p, 's: 'p, P: 'p + 's> {
    /// The parser that is checking the nest limit.
    p: &'p ParserI<'s, P>,
    /// The current depth while walking an Ast.
    depth: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
}
#[derive(Clone, Debug)]
pub struct Parser {
    /// The current position of the parser.
    pos: Cell<Position>,
    /// The current capture index.
    capture_index: Cell<u32>,
    /// The maximum number of open parens/brackets allowed. If the parser
    /// exceeds this number, then an error is returned.
    nest_limit: u32,
    /// Whether to support octal syntax or not. When `false`, the parser will
    /// return an error helpfully pointing out that backreferences are not
    /// supported.
    octal: bool,
    /// The initial setting for `ignore_whitespace` as provided by
    /// Th`ParserBuilder`. is is used when reseting the parser's state.
    initial_ignore_whitespace: bool,
    /// Whether whitespace should be ignored. When enabled, comments are
    /// also permitted.
    ignore_whitespace: Cell<bool>,
    /// A list of comments, in order of appearance.
    comments: RefCell<Vec<ast::Comment>>,
    /// A stack of grouped sub-expressions, including alternations.
    stack_group: RefCell<Vec<GroupState>>,
    /// A stack of nested character classes. This is only non-empty when
    /// parsing a class.
    stack_class: RefCell<Vec<ClassState>>,
    /// A sorted sequence of capture names. This is used to detect duplicate
    /// capture names and report an error if one is detected.
    capture_names: RefCell<Vec<ast::CaptureName>>,
    /// A scratch buffer used in various places. Mostly this is used to
    /// accumulate relevant characters from parts of a pattern.
    scratch: RefCell<String>,
}
#[derive(Clone, Debug)]
struct ParserI<'s, P> {
    /// The parser state/configuration.
    parser: P,
    /// The full regular expression provided by the user.
    pattern: &'s str,
}
#[derive(Clone, Debug)]
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
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
impl<'p, 's, P: Borrow<Parser>> NestLimiter<'p, 's, P> {
    fn new(p: &'p ParserI<'s, P>) -> NestLimiter<'p, 's, P> {}
    fn check(self, ast: &Ast) -> Result<()> {
        ast::visit(ast, self)
    }
    fn increment_depth(&mut self, span: &Span) -> Result<()> {}
    fn decrement_depth(&mut self) {}
}
