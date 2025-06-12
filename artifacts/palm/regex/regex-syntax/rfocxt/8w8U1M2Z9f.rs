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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
impl Literals {
    pub fn empty() -> Literals {}
    pub fn prefixes(expr: &Hir) -> Literals {}
    pub fn suffixes(expr: &Hir) -> Literals {}
    pub fn limit_size(&self) -> usize {
        self.limit_size
    }
    pub fn set_limit_size(&mut self, size: usize) -> &mut Literals {
        self.limit_size = size;
        self
    }
    pub fn limit_class(&self) -> usize {}
    pub fn set_limit_class(&mut self, size: usize) -> &mut Literals {}
    pub fn literals(&self) -> &[Literal] {}
    pub fn min_len(&self) -> Option<usize> {}
    pub fn all_complete(&self) -> bool {}
    pub fn any_complete(&self) -> bool {}
    pub fn contains_empty(&self) -> bool {}
    pub fn is_empty(&self) -> bool {
        self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
    }
    pub fn to_empty(&self) -> Literals {
        let mut lits = Literals::empty();
        lits.set_limit_size(self.limit_size).set_limit_class(self.limit_class);
        lits
    }
    pub fn longest_common_prefix(&self) -> &[u8] {}
    pub fn longest_common_suffix(&self) -> &[u8] {}
    pub fn trim_suffix(&self, num_bytes: usize) -> Option<Literals> {}
    pub fn unambiguous_prefixes(&self) -> Literals {}
    pub fn unambiguous_suffixes(&self) -> Literals {}
    pub fn union_prefixes(&mut self, expr: &Hir) -> bool {}
    pub fn union_suffixes(&mut self, expr: &Hir) -> bool {}
    pub fn union(&mut self, lits: Literals) -> bool {
        if self.num_bytes() + lits.num_bytes() > self.limit_size {
            return false;
        }
        if lits.is_empty() {
            self.lits.push(Literal::empty());
        } else {
            self.lits.extend(lits.lits);
        }
        true
    }
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
    pub fn cut(&mut self) {
        for lit in &mut self.lits {
            lit.cut();
        }
    }
    pub fn reverse(&mut self) {}
    pub fn clear(&mut self) {}
    fn remove_complete(&mut self) -> Vec<Literal> {}
    fn num_bytes(&self) -> usize {}
    fn class_exceeds_limits(&self, size: usize) -> bool {}
}
fn alternate_literals<F: FnMut(&Hir, &mut Literals)>(
    es: &[Hir],
    lits: &mut Literals,
    mut f: F,
) {
    let mut lits2 = lits.to_empty();
    for e in es {
        let mut lits3 = lits.to_empty();
        lits3.set_limit_size(lits.limit_size() / 5);
        f(e, &mut lits3);
        if lits3.is_empty() || !lits2.union(lits3) {
            lits.cut();
            return;
        }
    }
    if !lits.cross_product(&lits2) {
        lits.cut();
    }
}
