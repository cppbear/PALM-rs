use ast;
use hir;
use Result;
#[derive(Clone, Debug, Default)]
pub struct ParserBuilder {
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    allow_invalid_utf8: bool,
    flags: Flags,
}
#[derive(Clone, Debug)]
pub struct ParserBuilder {
    ignore_whitespace: bool,
    nest_limit: u32,
    octal: bool,
}
impl ParserBuilder {
    pub fn new() -> ParserBuilder {}
    pub fn build(&self) -> Parser {}
    pub fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder {}
    pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.case_insensitive(yes);
        self
    }
    pub fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {}
}
impl TranslatorBuilder {
    pub fn new() -> TranslatorBuilder {}
    pub fn build(&self) -> Translator {}
    pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.case_insensitive = if yes { Some(true) } else { None };
        self
    }
    pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder {}
}
