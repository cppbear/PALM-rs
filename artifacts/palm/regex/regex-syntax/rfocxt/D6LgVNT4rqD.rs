use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
#[derive(Clone, Eq, PartialEq)]
pub struct Literals {
    lits: Vec<Literal>,
    limit_size: usize,
    limit_class: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    /// The kind of this repetition operator.
    pub kind: RepetitionKind,
    /// Whether this repetition operator is greedy or not. A greedy operator
    /// will match as much as it can. A non-greedy operator will match as
    /// little as it can.
    ///
    /// Typically, operators are greedy by default and are only non-greedy when
    /// a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
    /// not. However, this can be inverted via the `U` "ungreedy" flag.
    pub greedy: bool,
    /// The expression being repeated.
    pub hir: Box<Hir>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionKind {
    /// Matches a sub-expression zero or one times.
    ZeroOrOne,
    /// Matches a sub-expression zero or more times.
    ZeroOrMore,
    /// Matches a sub-expression one or more times.
    OneOrMore,
    /// Matches a sub-expression within a bounded range of times.
    Range(RepetitionRange),
}
impl Hir {
    pub fn kind(&self) -> &HirKind {}
    pub fn into_kind(mut self) -> HirKind {}
    pub fn empty() -> Hir {}
    pub fn literal(lit: Literal) -> Hir {}
    pub fn class(class: Class) -> Hir {}
    pub fn anchor(anchor: Anchor) -> Hir {}
    pub fn word_boundary(word_boundary: WordBoundary) -> Hir {}
    pub fn repetition(rep: Repetition) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(rep.hir.is_always_utf8());
        info.set_all_assertions(rep.hir.is_all_assertions());
        info.set_anchored_start(!rep.is_match_empty() && rep.hir.is_anchored_start());
        info.set_anchored_end(!rep.is_match_empty() && rep.hir.is_anchored_end());
        info.set_any_anchored_start(rep.hir.is_any_anchored_start());
        info.set_any_anchored_end(rep.hir.is_any_anchored_end());
        info.set_match_empty(rep.is_match_empty() || rep.hir.is_match_empty());
        Hir {
            kind: HirKind::Repetition(rep),
            info: info,
        }
    }
    pub fn group(group: Group) -> Hir {}
    pub fn concat(mut exprs: Vec<Hir>) -> Hir {
        match exprs.len() {
            0 => Hir::empty(),
            1 => exprs.pop().unwrap(),
            _ => {
                let mut info = HirInfo::new();
                info.set_always_utf8(true);
                info.set_all_assertions(true);
                info.set_any_anchored_start(false);
                info.set_any_anchored_end(false);
                info.set_match_empty(true);
                for e in &exprs {
                    let x = info.is_always_utf8() && e.is_always_utf8();
                    info.set_always_utf8(x);
                    let x = info.is_all_assertions() && e.is_all_assertions();
                    info.set_all_assertions(x);
                    let x = info.is_any_anchored_start() || e.is_any_anchored_start();
                    info.set_any_anchored_start(x);
                    let x = info.is_any_anchored_end() || e.is_any_anchored_end();
                    info.set_any_anchored_end(x);
                    let x = info.is_match_empty() && e.is_match_empty();
                    info.set_match_empty(x);
                }
                info.set_anchored_start(
                    exprs
                        .iter()
                        .take_while(|e| {
                            e.is_anchored_start() || e.is_all_assertions()
                        })
                        .any(|e| { e.is_anchored_start() }),
                );
                info.set_anchored_end(
                    exprs
                        .iter()
                        .rev()
                        .take_while(|e| { e.is_anchored_end() || e.is_all_assertions() })
                        .any(|e| { e.is_anchored_end() }),
                );
                Hir {
                    kind: HirKind::Concat(exprs),
                    info: info,
                }
            }
        }
    }
    pub fn alternation(mut exprs: Vec<Hir>) -> Hir {}
    pub fn dot(bytes: bool) -> Hir {}
    pub fn any(bytes: bool) -> Hir {}
    pub fn is_always_utf8(&self) -> bool {}
    pub fn is_all_assertions(&self) -> bool {}
    pub fn is_anchored_start(&self) -> bool {}
    pub fn is_anchored_end(&self) -> bool {}
    pub fn is_any_anchored_start(&self) -> bool {}
    pub fn is_any_anchored_end(&self) -> bool {}
    pub fn is_match_empty(&self) -> bool {}
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
    pub fn contains_empty(&self) -> bool {
        self.lits.iter().any(|lit| lit.is_empty())
    }
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
fn repeat_range_literals<F: FnMut(&Hir, &mut Literals)>(
    e: &Hir,
    min: u32,
    max: Option<u32>,
    greedy: bool,
    lits: &mut Literals,
    mut f: F,
) {
    if min == 0 {
        f(
            &Hir::repetition(hir::Repetition {
                kind: hir::RepetitionKind::ZeroOrMore,
                greedy: greedy,
                hir: Box::new(e.clone()),
            }),
            lits,
        );
    } else {
        if min > 0 {
            let n = cmp::min(lits.limit_size, min as usize);
            let es = iter::repeat(e.clone()).take(n).collect();
            f(&Hir::concat(es), lits);
            if n < min as usize || lits.contains_empty() {
                lits.cut();
            }
        }
        if max.map_or(true, |max| min < max) {
            lits.cut();
        }
    }
}
