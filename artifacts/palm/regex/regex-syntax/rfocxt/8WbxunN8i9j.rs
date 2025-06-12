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
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Eq, Ord)]
pub struct Literal {
    v: Vec<u8>,
    cut: bool,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
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
#[derive(Debug)]
pub struct ClassUnicodeIter<'a>(IntervalSetIter<'a, ClassUnicodeRange>);
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
    pub fn unambiguous_prefixes(&self) -> Literals {}
    pub fn unambiguous_suffixes(&self) -> Literals {}
    pub fn union_prefixes(&mut self, expr: &Hir) -> bool {}
    pub fn union_suffixes(&mut self, expr: &Hir) -> bool {}
    pub fn union(&mut self, lits: Literals) -> bool {}
    pub fn cross_product(&mut self, lits: &Literals) -> bool {}
    pub fn cross_add(&mut self, bytes: &[u8]) -> bool {}
    pub fn add(&mut self, lit: Literal) -> bool {}
    pub fn add_char_class(&mut self, cls: &hir::ClassUnicode) -> bool {}
    fn add_char_class_reverse(&mut self, cls: &hir::ClassUnicode) -> bool {}
    fn _add_char_class(&mut self, cls: &hir::ClassUnicode, reverse: bool) -> bool {
        use std::char;
        if self.class_exceeds_limits(cls_char_count(cls)) {
            return false;
        }
        let mut base = self.remove_complete();
        if base.is_empty() {
            base = vec![Literal::empty()];
        }
        for r in cls.iter() {
            let (s, e) = (r.start as u32, r.end as u32 + 1);
            for c in (s..e).filter_map(char::from_u32) {
                for mut lit in base.clone() {
                    let mut bytes = c.to_string().into_bytes();
                    if reverse {
                        bytes.reverse();
                    }
                    lit.extend(&bytes);
                    self.lits.push(lit);
                }
            }
        }
        true
    }
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
    fn num_bytes(&self) -> usize {}
    fn class_exceeds_limits(&self, size: usize) -> bool {
        if size > self.limit_class {
            return true;
        }
        let new_byte_count = if self.lits.is_empty() {
            size
        } else {
            self.lits
                .iter()
                .fold(
                    0,
                    |accum, lit| {
                        accum + if lit.is_cut() { 0 } else { (lit.len() + 1) * size }
                    },
                )
        };
        new_byte_count > self.limit_size
    }
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter {
        ClassUnicodeIter(self.set.iter())
    }
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassUnicode) {}
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
}
impl Literal {
    pub fn new(bytes: Vec<u8>) -> Literal {}
    pub fn empty() -> Literal {
        Literal { v: vec![], cut: false }
    }
    pub fn is_cut(&self) -> bool {}
    pub fn cut(&mut self) {}
}
fn cls_char_count(cls: &hir::ClassUnicode) -> usize {
    cls.iter().map(|&r| 1 + (r.end as u32) - (r.start as u32)).sum::<u32>() as usize
}
