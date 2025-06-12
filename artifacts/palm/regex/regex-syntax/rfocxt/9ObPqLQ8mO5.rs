type Result<T> = result::Result<T, ast::Error>;
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::mem;
use std::result;
use ast::{self, Ast, Position, Span};
use either::Either;
use is_meta_character;
#[derive(Clone, Debug)]
pub struct ParserBuilder {
    ignore_whitespace: bool,
    nest_limit: u32,
    octal: bool,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position {
    /// The absolute offset of this position, starting at `0` from the
    /// beginning of the regular expression pattern string.
    pub offset: usize,
    /// The line number, starting at `1`.
    pub line: usize,
    /// The approximate column number, starting at `1`.
    pub column: usize,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureName {
    /// The span of this capture name.
    pub span: Span,
    /// The capture name.
    pub name: String,
    /// The capture index.
    pub index: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comment {
    /// The span of this comment, including the beginning `#` and ending `\n`.
    pub span: Span,
    /// The comment text, starting with the first character following the `#`
    /// and ending with the last character preceding the `\n`.
    pub comment: String,
}
#[derive(Clone, Debug)]
enum GroupState {
    /// This state is pushed whenever an opening group is found.
    Group {
        /// The concatenation immediately preceding the opening group.
        concat: ast::Concat,
        /// The group that has been opened. Its sub-AST is always empty.
        group: ast::Group,
        /// Whether this group has the `x` flag enabled or not.
        ignore_whitespace: bool,
    },
    /// This state is pushed whenever a new alternation branch is found. If
    /// an alternation branch is found and this state is at the top of the
    /// stack, then this state should be modified to include the new
    /// alternation.
    Alternation(ast::Alternation),
}
#[derive(Clone, Debug)]
enum ClassState {
    /// This state is pushed whenever an opening bracket is found.
    Open {
        /// The union of class items immediately preceding this class.
        union: ast::ClassSetUnion,
        /// The class that has been opened. Typically this just corresponds
        /// to the `[`, but it can also include `[^` since `^` indicates
        /// negation of the class.
        set: ast::ClassBracketed,
    },
    /// This state is pushed when a operator is seen. When popped, the stored
    /// set becomes the left hand side of the operator.
    Op {
        /// The type of the operation, i.e., &&, -- or ~~.
        kind: ast::ClassSetBinaryOpKind,
        /// The left-hand side of the operator.
        lhs: ast::ClassSet,
    },
}
impl ParserBuilder {
    pub fn new() -> ParserBuilder {}
    pub fn build(&self) -> Parser {
        Parser {
            pos: Cell::new(Position {
                offset: 0,
                line: 1,
                column: 1,
            }),
            capture_index: Cell::new(0),
            nest_limit: self.nest_limit,
            octal: self.octal,
            initial_ignore_whitespace: self.ignore_whitespace,
            ignore_whitespace: Cell::new(self.ignore_whitespace),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        }
    }
    pub fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {}
}
