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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
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
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assertion {
    /// The span of this assertion.
    pub span: Span,
    /// The assertion kind, e.g., `\b` or `^`.
    pub kind: AssertionKind,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssertionKind {
    /// `^`
    StartLine,
    /// `$`
    EndLine,
    /// `\A`
    StartText,
    /// `\z`
    EndText,
    /// `\b`
    WordBoundary,
    /// `\B`
    NotWordBoundary,
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum Primitive {
    Literal(ast::Literal),
    Assertion(ast::Assertion),
    Dot(Span),
    Perl(ast::ClassPerl),
    Unicode(ast::ClassUnicode),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiteralKind {
    /// The literal is written verbatim, e.g., `a` or `☃`.
    Verbatim,
    /// The literal is written as an escape because it is punctuation, e.g.,
    /// `\*` or `\[`.
    Punctuation,
    /// The literal is written as an octal escape, e.g., `\141`.
    Octal,
    /// The literal is written as a hex code with a fixed number of digits
    /// depending on the type of the escape, e.g., `\x61` or or `\u0061` or
    /// `\U00000061`.
    HexFixed(HexLiteralKind),
    /// The literal is written as a hex code with a bracketed number of
    /// digits. The only restriction is that the bracketed hex code must refer
    /// to a valid Unicode scalar value.
    HexBrace(HexLiteralKind),
    /// The literal is written as a specially recognized escape, e.g., `\f`
    /// or `\n`.
    Special(SpecialLiteralKind),
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
    fn parse_primitive(&self) -> Result<Primitive> {
        match self.char() {
            '\\' => self.parse_escape(),
            '.' => {
                let ast = Primitive::Dot(self.span_char());
                self.bump();
                Ok(ast)
            }
            '^' => {
                let ast = Primitive::Assertion(ast::Assertion {
                    span: self.span_char(),
                    kind: ast::AssertionKind::StartLine,
                });
                self.bump();
                Ok(ast)
            }
            '$' => {
                let ast = Primitive::Assertion(ast::Assertion {
                    span: self.span_char(),
                    kind: ast::AssertionKind::EndLine,
                });
                self.bump();
                Ok(ast)
            }
            c => {
                let ast = Primitive::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: c,
                });
                self.bump();
                Ok(ast)
            }
        }
    }
    fn parse_escape(&self) -> Result<Primitive> {
        assert_eq!(self.char(), '\\');
        let start = self.pos();
        if !self.bump() {
            return Err(
                self
                    .error(
                        Span::new(start, self.pos()),
                        ast::ErrorKind::EscapeUnexpectedEof,
                    ),
            );
        }
        let c = self.char();
        match c {
            '0'..='7' => {
                if !self.parser().octal {
                    return Err(
                        self
                            .error(
                                Span::new(start, self.span_char().end),
                                ast::ErrorKind::UnsupportedBackreference,
                            ),
                    );
                }
                let mut lit = self.parse_octal();
                lit.span.start = start;
                return Ok(Primitive::Literal(lit));
            }
            '8'..='9' if !self.parser().octal => {
                return Err(
                    self
                        .error(
                            Span::new(start, self.span_char().end),
                            ast::ErrorKind::UnsupportedBackreference,
                        ),
                );
            }
            'x' | 'u' | 'U' => {
                let mut lit = self.parse_hex()?;
                lit.span.start = start;
                return Ok(Primitive::Literal(lit));
            }
            'p' | 'P' => {
                let mut cls = self.parse_unicode_class()?;
                cls.span.start = start;
                return Ok(Primitive::Unicode(cls));
            }
            'd' | 's' | 'w' | 'D' | 'S' | 'W' => {
                let mut cls = self.parse_perl_class();
                cls.span.start = start;
                return Ok(Primitive::Perl(cls));
            }
            _ => {}
        }
        self.bump();
        let span = Span::new(start, self.pos());
        if is_meta_character(c) {
            return Ok(
                Primitive::Literal(ast::Literal {
                    span: span,
                    kind: ast::LiteralKind::Punctuation,
                    c: c,
                }),
            );
        }
        let special = |kind, c| Ok(
            Primitive::Literal(ast::Literal {
                span: span,
                kind: ast::LiteralKind::Special(kind),
                c: c,
            }),
        );
        match c {
            'a' => special(ast::SpecialLiteralKind::Bell, '\x07'),
            'f' => special(ast::SpecialLiteralKind::FormFeed, '\x0C'),
            't' => special(ast::SpecialLiteralKind::Tab, '\t'),
            'n' => special(ast::SpecialLiteralKind::LineFeed, '\n'),
            'r' => special(ast::SpecialLiteralKind::CarriageReturn, '\r'),
            'v' => special(ast::SpecialLiteralKind::VerticalTab, '\x0B'),
            ' ' if self.ignore_whitespace() => {
                special(ast::SpecialLiteralKind::Space, ' ')
            }
            'A' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span: span,
                        kind: ast::AssertionKind::StartText,
                    }),
                )
            }
            'z' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span: span,
                        kind: ast::AssertionKind::EndText,
                    }),
                )
            }
            'b' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span: span,
                        kind: ast::AssertionKind::WordBoundary,
                    }),
                )
            }
            'B' => {
                Ok(
                    Primitive::Assertion(ast::Assertion {
                        span: span,
                        kind: ast::AssertionKind::NotWordBoundary,
                    }),
                )
            }
            _ => Err(self.error(span, ast::ErrorKind::EscapeUnrecognized)),
        }
    }
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
    fn parse_perl_class(&self) -> ast::ClassPerl {}
}
