type Result<T> = result::Result<T, ast::Error>;
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::mem;
use std::result;
use ast::{self, Ast, Position, Span};
use either::Either;
use is_meta_character;
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
    pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
        self.octal = yes;
        self
    }
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {}
}
