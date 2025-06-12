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
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn new(parser: P, pattern: &'s str) -> ParserI<'s, P> {}
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
    fn bump(&self) -> bool {
        if self.is_eof() {
            return false;
        }
        let Position { mut offset, mut line, mut column } = self.pos();
        if self.char() == '\n' {
            line = line.checked_add(1).unwrap();
            column = 1;
        } else {
            column = column.checked_add(1).unwrap();
        }
        offset += self.char().len_utf8();
        self.parser()
            .pos
            .set(Position {
                offset: offset,
                line: line,
                column: column,
            });
        self.pattern()[self.offset()..].chars().next().is_some()
    }
    fn bump_if(&self, prefix: &str) -> bool {}
    fn is_lookaround_prefix(&self) -> bool {}
    fn bump_and_bump_space(&self) -> bool {
        if !self.bump() {
            return false;
        }
        self.bump_space();
        !self.is_eof()
    }
    fn bump_space(&self) {
        if !self.ignore_whitespace() {
            return;
        }
        while !self.is_eof() {
            if self.char().is_whitespace() {
                self.bump();
            } else if self.char() == '#' {
                let start = self.pos();
                let mut comment_text = String::new();
                self.bump();
                while !self.is_eof() {
                    let c = self.char();
                    self.bump();
                    if c == '\n' {
                        break;
                    }
                    comment_text.push(c);
                }
                let comment = ast::Comment {
                    span: Span::new(start, self.pos()),
                    comment: comment_text,
                };
                self.parser().comments.borrow_mut().push(comment);
            } else {
                break;
            }
        }
    }
    fn peek(&self) -> Option<char> {}
    fn peek_space(&self) -> Option<char> {}
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
