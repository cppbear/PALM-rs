use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
#[derive(Clone, Eq, PartialEq)]
pub struct Literals {
    lits: Vec<Literal>,
    limit_size: usize,
    limit_class: usize,
}
#[derive(Clone, Eq, Ord)]
pub struct Literal {
    v: Vec<u8>,
    cut: bool,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    /// A single character represented by a Unicode scalar value.
    Unicode(char),
    /// A single character represented by an arbitrary byte.
    Byte(u8),
}
impl Literals {
    pub fn empty() -> Literals {}
    pub fn prefixes(expr: &Hir) -> Literals {}
    pub fn suffixes(expr: &Hir) -> Literals {}
    pub fn limit_size(&self) -> usize {}
    pub fn set_limit_size(&mut self, size: usize) -> &mut Literals {}
    pub fn limit_class(&self) -> usize {}
    pub fn set_limit_class(&mut self, size: usize) -> &mut Literals {}
    pub fn literals(&self) -> &[Literal] {
        &self.lits
    }
    pub fn min_len(&self) -> Option<usize> {}
    pub fn all_complete(&self) -> bool {}
    pub fn any_complete(&self) -> bool {
        self.lits.iter().any(|lit| !lit.is_cut())
    }
    pub fn contains_empty(&self) -> bool {}
    pub fn is_empty(&self) -> bool {
        self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
    }
    pub fn to_empty(&self) -> Literals {}
    pub fn longest_common_prefix(&self) -> &[u8] {}
    pub fn longest_common_suffix(&self) -> &[u8] {}
    pub fn trim_suffix(&self, num_bytes: usize) -> Option<Literals> {}
    pub fn unambiguous_prefixes(&self) -> Literals {}
    pub fn unambiguous_suffixes(&self) -> Literals {}
    pub fn union_prefixes(&mut self, expr: &Hir) -> bool {}
    pub fn union_suffixes(&mut self, expr: &Hir) -> bool {}
    pub fn union(&mut self, lits: Literals) -> bool {}
    pub fn cross_product(&mut self, lits: &Literals) -> bool {
        if lits.is_empty() {
            return true;
        }
        let mut size_after;
        if self.is_empty() || !self.any_complete() {
            size_after = self.num_bytes();
            for lits_lit in lits.literals() {
                size_after += lits_lit.len();
            }
        } else {
            size_after = self
                .lits
                .iter()
                .fold(
                    0,
                    |accum, lit| { accum + if lit.is_cut() { lit.len() } else { 0 } },
                );
            for lits_lit in lits.literals() {
                for self_lit in self.literals() {
                    if !self_lit.is_cut() {
                        size_after += self_lit.len() + lits_lit.len();
                    }
                }
            }
        }
        if size_after > self.limit_size {
            return false;
        }
        let mut base = self.remove_complete();
        if base.is_empty() {
            base = vec![Literal::empty()];
        }
        for lits_lit in lits.literals() {
            for mut self_lit in base.clone() {
                self_lit.extend(&**lits_lit);
                self_lit.cut = lits_lit.cut;
                self.lits.push(self_lit);
            }
        }
        true
    }
    pub fn cross_add(&mut self, bytes: &[u8]) -> bool {}
    pub fn add(&mut self, lit: Literal) -> bool {}
    pub fn add_char_class(&mut self, cls: &hir::ClassUnicode) -> bool {}
    fn add_char_class_reverse(&mut self, cls: &hir::ClassUnicode) -> bool {}
    fn _add_char_class(&mut self, cls: &hir::ClassUnicode, reverse: bool) -> bool {}
    pub fn add_byte_class(&mut self, cls: &hir::ClassBytes) -> bool {}
    pub fn cut(&mut self) {}
    pub fn reverse(&mut self) {}
    pub fn clear(&mut self) {}
    fn remove_complete(&mut self) -> Vec<Literal> {
        let mut base = vec![];
        for lit in mem::replace(&mut self.lits, vec![]) {
            if lit.is_cut() {
                self.lits.push(lit);
            } else {
                base.push(lit);
            }
        }
        base
    }
    fn num_bytes(&self) -> usize {
        self.lits.iter().fold(0, |accum, lit| accum + lit.len())
    }
    fn class_exceeds_limits(&self, size: usize) -> bool {}
}
impl Literal {
    pub fn new(bytes: Vec<u8>) -> Literal {}
    pub fn empty() -> Literal {
        Literal { v: vec![], cut: false }
    }
    pub fn is_cut(&self) -> bool {
        self.cut
    }
    pub fn cut(&mut self) {}
}
