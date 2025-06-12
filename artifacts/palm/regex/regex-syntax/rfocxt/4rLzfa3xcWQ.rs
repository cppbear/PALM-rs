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
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
}
#[derive(Clone, Eq, Ord)]
pub struct Literal {
    v: Vec<u8>,
    cut: bool,
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
    pub fn literals(&self) -> &[Literal] {}
    pub fn min_len(&self) -> Option<usize> {}
    pub fn all_complete(&self) -> bool {}
    pub fn any_complete(&self) -> bool {}
    pub fn contains_empty(&self) -> bool {}
    pub fn is_empty(&self) -> bool {}
    pub fn to_empty(&self) -> Literals {}
    pub fn longest_common_prefix(&self) -> &[u8] {}
    pub fn longest_common_suffix(&self) -> &[u8] {}
    pub fn trim_suffix(&self, num_bytes: usize) -> Option<Literals> {}
    pub fn unambiguous_prefixes(&self) -> Literals {
        if self.lits.is_empty() {
            return self.to_empty();
        }
        let mut old: Vec<Literal> = self.lits.iter().cloned().collect();
        let mut new = self.to_empty();
        'OUTER: while let Some(mut candidate) = old.pop() {
            if candidate.is_empty() {
                continue;
            }
            if new.lits.is_empty() {
                new.lits.push(candidate);
                continue;
            }
            for lit2 in &mut new.lits {
                if lit2.is_empty() {
                    continue;
                }
                if &candidate == lit2 {
                    candidate.cut = candidate.cut || lit2.cut;
                    lit2.cut = candidate.cut;
                    continue 'OUTER;
                }
                if candidate.len() < lit2.len() {
                    if let Some(i) = position(&candidate, &lit2) {
                        candidate.cut();
                        let mut lit3 = lit2.clone();
                        lit3.truncate(i);
                        lit3.cut();
                        old.push(lit3);
                        lit2.clear();
                    }
                } else {
                    if let Some(i) = position(&lit2, &candidate) {
                        lit2.cut();
                        let mut new_candidate = candidate.clone();
                        new_candidate.truncate(i);
                        new_candidate.cut();
                        old.push(new_candidate);
                        candidate.clear();
                    }
                }
                if candidate.is_empty() {
                    continue 'OUTER;
                }
            }
            new.lits.push(candidate);
        }
        new.lits.retain(|lit| !lit.is_empty());
        new.lits.sort();
        new.lits.dedup();
        new
    }
    pub fn unambiguous_suffixes(&self) -> Literals {
        let mut lits = self.clone();
        lits.reverse();
        let mut unamb = lits.unambiguous_prefixes();
        unamb.reverse();
        unamb
    }
    pub fn union_prefixes(&mut self, expr: &Hir) -> bool {}
    pub fn union_suffixes(&mut self, expr: &Hir) -> bool {}
    pub fn union(&mut self, lits: Literals) -> bool {}
    pub fn cross_product(&mut self, lits: &Literals) -> bool {}
    pub fn cross_add(&mut self, bytes: &[u8]) -> bool {}
    pub fn add(&mut self, lit: Literal) -> bool {}
    pub fn add_char_class(&mut self, cls: &hir::ClassUnicode) -> bool {}
    fn add_char_class_reverse(&mut self, cls: &hir::ClassUnicode) -> bool {}
    fn _add_char_class(&mut self, cls: &hir::ClassUnicode, reverse: bool) -> bool {}
    pub fn add_byte_class(&mut self, cls: &hir::ClassBytes) -> bool {}
    pub fn cut(&mut self) {}
    pub fn reverse(&mut self) {
        for lit in &mut self.lits {
            lit.reverse();
        }
    }
    pub fn clear(&mut self) {}
    fn remove_complete(&mut self) -> Vec<Literal> {}
    fn num_bytes(&self) -> usize {}
    fn class_exceeds_limits(&self, size: usize) -> bool {}
}
