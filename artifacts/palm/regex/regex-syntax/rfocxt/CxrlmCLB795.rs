type Result<T> = result::Result<T, ast::Error>;
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::mem;
use std::result;
use ast::{self, Ast, Position, Span};
use either::Either;
use is_meta_character;
#[derive(Clone, Debug)]
struct ParserI<'s, P> {
    /// The parser state/configuration.
    parser: P,
    /// The full regular expression provided by the user.
    pattern: &'s str,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
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
pub struct ClassSetBinaryOp {
    /// The span of this operation. e.g., the `a-z--[h-p]` in `[a-z--h-p]`.
    pub span: Span,
    /// The type of this set operation.
    pub kind: ClassSetBinaryOpKind,
    /// The left hand side of the operation.
    pub lhs: Box<ClassSet>,
    /// The right hand side of the operation.
    pub rhs: Box<ClassSet>,
}
#[derive(Clone, Debug)]
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
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
pub enum ClassSet {
    /// An item, which can be a single literal, range, nested character class
    /// or a union of items.
    Item(ClassSetItem),
    /// A single binary operation (i.e., &&, -- or ~~).
    BinaryOp(ClassSetBinaryOp),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassSetBinaryOpKind {
    /// The intersection of two sets, e.g., `\pN&&[a-z]`.
    Intersection,
    /// The difference of two sets, e.g., `\pN--[0-9]`.
    Difference,
    /// The symmetric difference of two sets. The symmetric difference is the
    /// set of elements belonging to one but not both sets.
    /// e.g., `[\pL~~[:ascii:]]`.
    SymmetricDifference,
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
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn new(parser: P, pattern: &'s str) -> ParserI<'s, P> {}
    fn parser(&self) -> &Parser {
        self.parser.borrow()
    }
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
    fn pop_class_op(&self, rhs: ast::ClassSet) -> ast::ClassSet {
        let mut stack = self.parser().stack_class.borrow_mut();
        let (kind, lhs) = match stack.pop() {
            Some(ClassState::Op { kind, lhs }) => (kind, lhs),
            Some(state @ ClassState::Open { .. }) => {
                stack.push(state);
                return rhs;
            }
            None => unreachable!(),
        };
        let span = Span::new(lhs.span().start, rhs.span().end);
        ast::ClassSet::BinaryOp(ast::ClassSetBinaryOp {
            span: span,
            kind: kind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}
impl ClassSet {
    pub fn union(ast: ClassSetUnion) -> ClassSet {}
    pub fn span(&self) -> &Span {
        match *self {
            ClassSet::Item(ref x) => x.span(),
            ClassSet::BinaryOp(ref x) => &x.span,
        }
    }
    fn is_empty(&self) -> bool {}
}
impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start: start, end: end }
    }
    pub fn splat(pos: Position) -> Span {}
    pub fn with_start(self, pos: Position) -> Span {}
    pub fn with_end(self, pos: Position) -> Span {}
    pub fn is_one_line(&self) -> bool {}
    pub fn is_empty(&self) -> bool {}
}
