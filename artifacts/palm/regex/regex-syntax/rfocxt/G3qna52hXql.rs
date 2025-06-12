use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
#[derive(Clone, Eq, Ord)]
pub struct Literal {
    v: Vec<u8>,
    cut: bool,
}
impl Literal {
    pub fn new(bytes: Vec<u8>) -> Literal {}
    pub fn empty() -> Literal {}
    pub fn is_cut(&self) -> bool {}
    pub fn cut(&mut self) {
        self.cut = true;
    }
}
