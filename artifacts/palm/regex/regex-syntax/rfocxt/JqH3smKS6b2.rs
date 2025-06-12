use ast;
use hir;
use Result;
#[derive(Clone, Debug)]
pub struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
}
#[derive(Clone, Debug, Default)]
pub struct ParserBuilder {
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub struct Translator {
    /// Our call stack, but on the heap.
    stack: RefCell<Vec<HirFrame>>,
    /// The current flag settings.
    flags: Cell<Flags>,
    /// Whether we're allowed to produce HIR that can match arbitrary bytes.
    allow_invalid_utf8: bool,
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
impl Parser {
    pub fn new() -> Parser {
        ParserBuilder::new().build()
    }
    pub fn parse(&mut self, pattern: &str) -> Result<hir::Hir> {}
}
impl ParserBuilder {
    pub fn new() -> ParserBuilder {
        ParserBuilder::default()
    }
    pub fn build(&self) -> Parser {
        Parser {
            ast: self.ast.build(),
            hir: self.hir.build(),
        }
    }
    pub fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {}
}
