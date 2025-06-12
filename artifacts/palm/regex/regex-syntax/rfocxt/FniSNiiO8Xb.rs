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
impl ops::DerefMut for Literal {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.v
    }
}
