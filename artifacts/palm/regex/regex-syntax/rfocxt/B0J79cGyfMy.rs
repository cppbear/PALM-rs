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
#[derive(Clone, Debug)]
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassPerl {
    /// The span of this class.
    pub span: Span,
    /// The kind of Perl class.
    pub kind: ClassPerlKind,
    /// Whether the class is negated or not. e.g., `\d` is not negated but
    /// `\D` is.
    pub negated: bool,
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
pub enum ClassPerlKind {
    /// Decimal numbers.
    Digit,
    /// Whitespace.
    Space,
    /// Word characters.
    Word,
}
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn parse(&self) -> Result<Ast> {}
    fn parse_with_comments(&self) -> Result<ast::WithComments> {}
    fn parse_uncounted_repetition(
        &self,
        mut concat: ast::Concat,
        kind: ast::RepetitionKind,
    ) -> Result<ast::Concat> {}
    fn parse_counted_repetition(&self, mut concat: ast::Concat) -> Result<ast::Concat> {}
    fn parse_group(&self) -> Result<Either<ast::SetFlags, ast::Group>> {}
    fn parse_capture_name(&self, capture_index: u32) -> Result<ast::CaptureName> {}
    fn parse_flags(&self) -> Result<ast::Flags> {}
    fn parse_flag(&self) -> Result<ast::Flag> {}
    fn parse_primitive(&self) -> Result<Primitive> {}
    fn parse_escape(&self) -> Result<Primitive> {}
    fn parse_octal(&self) -> ast::Literal {}
    fn parse_hex(&self) -> Result<ast::Literal> {}
    fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {}
    fn parse_hex_brace(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {}
    fn parse_decimal(&self) -> Result<u32> {}
    fn parse_set_class(&self) -> Result<ast::Class> {}
    fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {}
    fn parse_set_class_item(&self) -> Result<Primitive> {}
    fn parse_set_class_open(&self) -> Result<(ast::ClassBracketed, ast::ClassSetUnion)> {}
    fn maybe_parse_ascii_class(&self) -> Option<ast::ClassAscii> {}
    fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {}
    fn parse_perl_class(&self) -> ast::ClassPerl {
        let c = self.char();
        let span = self.span_char();
        self.bump();
        let (negated, kind) = match c {
            'd' => (false, ast::ClassPerlKind::Digit),
            'D' => (true, ast::ClassPerlKind::Digit),
            's' => (false, ast::ClassPerlKind::Space),
            'S' => (true, ast::ClassPerlKind::Space),
            'w' => (false, ast::ClassPerlKind::Word),
            'W' => (true, ast::ClassPerlKind::Word),
            c => panic!("expected valid Perl class but got '{}'", c),
        };
        ast::ClassPerl {
            span: span,
            kind: kind,
            negated: negated,
        }
    }
}
