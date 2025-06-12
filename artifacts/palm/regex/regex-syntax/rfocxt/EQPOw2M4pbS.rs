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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassAscii {
    /// The span of this class.
    pub span: Span,
    /// The kind of ASCII class.
    pub kind: ClassAsciiKind,
    /// Whether the class is negated or not. e.g., `[[:alpha:]]` is not negated
    /// but `[[:^alpha:]]` is.
    pub negated: bool,
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
pub enum ClassAsciiKind {
    /// `[0-9A-Za-z]`
    Alnum,
    /// `[A-Za-z]`
    Alpha,
    /// `[\x00-\x7F]`
    Ascii,
    /// `[ \t]`
    Blank,
    /// `[\x00-\x1F\x7F]`
    Cntrl,
    /// `[0-9]`
    Digit,
    /// `[!-~]`
    Graph,
    /// `[a-z]`
    Lower,
    /// `[ -~]`
    Print,
    /// `[!-/:-@\[-`{-~]`
    Punct,
    /// `[\t\n\v\f\r ]`
    Space,
    /// `[A-Z]`
    Upper,
    /// `[0-9A-Za-z_]`
    Word,
    /// `[0-9A-Fa-f]`
    Xdigit,
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
    fn maybe_parse_ascii_class(&self) -> Option<ast::ClassAscii> {
        assert_eq!(self.char(), '[');
        let start = self.pos();
        let mut negated = false;
        if !self.bump() || self.char() != ':' {
            self.parser().pos.set(start);
            return None;
        }
        if !self.bump() {
            self.parser().pos.set(start);
            return None;
        }
        if self.char() == '^' {
            negated = true;
            if !self.bump() {
                self.parser().pos.set(start);
                return None;
            }
        }
        let name_start = self.offset();
        while self.char() != ':' && self.bump() {}
        if self.is_eof() {
            self.parser().pos.set(start);
            return None;
        }
        let name = &self.pattern()[name_start..self.offset()];
        if !self.bump_if(":]") {
            self.parser().pos.set(start);
            return None;
        }
        let kind = match ast::ClassAsciiKind::from_name(name) {
            Some(kind) => kind,
            None => {
                self.parser().pos.set(start);
                return None;
            }
        };
        Some(ast::ClassAscii {
            span: Span::new(start, self.pos()),
            kind: kind,
            negated: negated,
        })
    }
    fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {}
    fn parse_perl_class(&self) -> ast::ClassPerl {}
}
impl ClassAsciiKind {
    pub fn from_name(name: &str) -> Option<ClassAsciiKind> {
        use self::ClassAsciiKind::*;
        match name {
            "alnum" => Some(Alnum),
            "alpha" => Some(Alpha),
            "ascii" => Some(Ascii),
            "blank" => Some(Blank),
            "cntrl" => Some(Cntrl),
            "digit" => Some(Digit),
            "graph" => Some(Graph),
            "lower" => Some(Lower),
            "print" => Some(Print),
            "punct" => Some(Punct),
            "space" => Some(Space),
            "upper" => Some(Upper),
            "word" => Some(Word),
            "xdigit" => Some(Xdigit),
            _ => None,
        }
    }
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
