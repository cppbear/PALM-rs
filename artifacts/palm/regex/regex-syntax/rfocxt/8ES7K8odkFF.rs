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
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
}
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn new(parser: P, pattern: &'s str) -> ParserI<'s, P> {}
    fn parser(&self) -> &Parser {}
    fn pattern(&self) -> &str {
        self.pattern.borrow()
    }
    fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {}
    fn offset(&self) -> usize {
        self.parser().pos.get().offset
    }
    fn line(&self) -> usize {}
    fn column(&self) -> usize {}
    fn next_capture_index(&self, span: Span) -> Result<u32> {}
    fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {}
    fn ignore_whitespace(&self) -> bool {
        self.parser().ignore_whitespace.get()
    }
    fn char(&self) -> char {
        self.char_at(self.offset())
    }
    fn char_at(&self, i: usize) -> char {}
    fn bump(&self) -> bool {}
    fn bump_if(&self, prefix: &str) -> bool {}
    fn is_lookaround_prefix(&self) -> bool {}
    fn bump_and_bump_space(&self) -> bool {}
    fn bump_space(&self) {}
    fn peek(&self) -> Option<char> {
        if self.is_eof() {
            return None;
        }
        self.pattern()[self.offset() + self.char().len_utf8()..].chars().next()
    }
    fn peek_space(&self) -> Option<char> {
        if !self.ignore_whitespace() {
            return self.peek();
        }
        if self.is_eof() {
            return None;
        }
        let mut start = self.offset() + self.char().len_utf8();
        let mut in_comment = false;
        for (i, c) in self.pattern()[start..].char_indices() {
            if c.is_whitespace() {
                continue;
            } else if !in_comment && c == '#' {
                in_comment = true;
            } else if in_comment && c == '\n' {
                in_comment = false;
            } else {
                start += i;
                break;
            }
        }
        self.pattern()[start..].chars().next()
    }
    fn is_eof(&self) -> bool {
        self.offset() == self.pattern().len()
    }
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
