use ast;
use hir;
use Result;
#[derive(Clone, Debug, Default)]
pub struct ParserBuilder {
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub struct ParserBuilder {
    ignore_whitespace: bool,
    nest_limit: u32,
    octal: bool,
}
#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    allow_invalid_utf8: bool,
    flags: Flags,
}
impl ParserBuilder {
    pub fn new() -> ParserBuilder {}
    pub fn build(&self) -> Parser {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
        self.octal = yes;
        self
    }
    pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {}
}
