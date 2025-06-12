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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithComments {
    /// The actual ast.
    pub ast: Ast,
    /// All comments found in the original regular expression.
    pub comments: Vec<Comment>,
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
impl<'s, P: Borrow<Parser>> ParserI<'s, P> {
    fn parse(&self) -> Result<Ast> {
        self.parse_with_comments().map(|astc| astc.ast)
    }
    fn parse_with_comments(&self) -> Result<ast::WithComments> {
        assert_eq!(self.offset(), 0, "parser can only be used once");
        self.parser().reset();
        let mut concat = ast::Concat {
            span: self.span(),
            asts: vec![],
        };
        loop {
            self.bump_space();
            if self.is_eof() {
                break;
            }
            match self.char() {
                '(' => concat = self.push_group(concat)?,
                ')' => concat = self.pop_group(concat)?,
                '|' => concat = self.push_alternate(concat)?,
                '[' => {
                    let class = self.parse_set_class()?;
                    concat.asts.push(Ast::Class(class));
                }
                '?' => {
                    concat = self
                        .parse_uncounted_repetition(
                            concat,
                            ast::RepetitionKind::ZeroOrOne,
                        )?;
                }
                '*' => {
                    concat = self
                        .parse_uncounted_repetition(
                            concat,
                            ast::RepetitionKind::ZeroOrMore,
                        )?;
                }
                '+' => {
                    concat = self
                        .parse_uncounted_repetition(
                            concat,
                            ast::RepetitionKind::OneOrMore,
                        )?;
                }
                '{' => {
                    concat = self.parse_counted_repetition(concat)?;
                }
                _ => concat.asts.push(self.parse_primitive()?.into_ast()),
            }
        }
        let ast = self.pop_group_end(concat)?;
        NestLimiter::new(self).check(&ast)?;
        Ok(ast::WithComments {
            ast: ast,
            comments: mem::replace(&mut *self.parser().comments.borrow_mut(), vec![]),
        })
    }
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
    fn parse_perl_class(&self) -> ast::ClassPerl {}
}
