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
impl ops::Deref for Literal {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.v
    }
}
