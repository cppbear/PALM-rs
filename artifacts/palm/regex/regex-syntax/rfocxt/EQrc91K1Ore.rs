type Result<T> = result::Result<T, ast::Error>;
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::mem;
use std::result;
use ast::{self, Ast, Position, Span};
use either::Either;
use is_meta_character;
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
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
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
impl Parser {
    pub fn new() -> Parser {}
    pub fn parse(&mut self, pattern: &str) -> Result<Ast> {
        ParserI::new(self, pattern).parse()
    }
    pub fn parse_with_comments(&mut self, pattern: &str) -> Result<ast::WithComments> {}
    fn reset(&self) {}
}
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn new(parser: P, pattern: &'s str) -> ParserI<'s, P> {
        ParserI {
            parser: parser,
            pattern: pattern,
        }
    }
    fn parser(&self) -> &Parser {}
    fn pattern(&self) -> &str {}
    fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {}
    fn offset(&self) -> usize {}
    fn line(&self) -> usize {}
    fn column(&self) -> usize {}
    fn next_capture_index(&self, span: Span) -> Result<u32> {}
    fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {}
    fn ignore_whitespace(&self) -> bool {}
    fn char(&self) -> char {
        self.char_at(self.offset())
    }
    fn char_at(&self, i: usize) -> char {}
    fn bump(&self) -> bool {}
    fn bump_if(&self, prefix: &str) -> bool {}
    fn is_lookaround_prefix(&self) -> bool {}
    fn bump_and_bump_space(&self) -> bool {}
    fn bump_space(&self) {}
    fn peek(&self) -> Option<char> {}
    fn peek_space(&self) -> Option<char> {}
    fn is_eof(&self) -> bool {}
    fn pos(&self) -> Position {}
    fn span(&self) -> Span {}
    fn span_char(&self) -> Span {}
    fn push_alternate(&self, mut concat: ast::Concat) -> Result<ast::Concat> {}
    fn push_or_add_alternation(&self, concat: ast::Concat) {}
    fn push_group(&self, mut concat: ast::Concat) -> Result<ast::Concat> {}
    fn pop_group(&self, mut group_concat: ast::Concat) -> Result<ast::Concat> {}
    fn pop_group_end(&self, mut concat: ast::Concat) -> Result<Ast> {}
    fn push_class_open(
        &self,
        parent_union: ast::ClassSetUnion,
    ) -> Result<ast::ClassSetUnion> {}
    fn pop_class(
        &self,
        nested_union: ast::ClassSetUnion,
    ) -> Result<Either<ast::ClassSetUnion, ast::Class>> {}
    fn unclosed_class_error(&self) -> ast::Error {}
    fn push_class_op(
        &self,
        next_kind: ast::ClassSetBinaryOpKind,
        next_union: ast::ClassSetUnion,
    ) -> ast::ClassSetUnion {}
    fn pop_class_op(&self, rhs: ast::ClassSet) -> ast::ClassSet {}
}
