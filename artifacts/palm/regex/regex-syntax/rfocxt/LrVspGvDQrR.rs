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
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
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
pub enum Literal {
    /// A single character represented by a Unicode scalar value.
    Unicode(char),
    /// A single character represented by an arbitrary byte.
    Byte(u8),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Class {
    /// A set of characters represented by Unicode scalar values.
    Unicode(ClassUnicode),
    /// A set of characters represented by arbitrary bytes (one byte per
    /// character).
    Bytes(ClassBytes),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirKind {
    /// The empty regular expression, which matches everything, including the
    /// empty string.
    Empty,
    /// A single literal character that matches exactly this character.
    Literal(Literal),
    /// A single character class that matches any of the characters in the
    /// class. A class can either consist of Unicode scalar values as
    /// characters, or it can use bytes.
    Class(Class),
    /// An anchor assertion. An anchor assertion match always has zero length.
    Anchor(Anchor),
    /// A word boundary assertion, which may or may not be Unicode aware. A
    /// word boundary assertion match always has zero length.
    WordBoundary(WordBoundary),
    /// A repetition operation applied to a child expression.
    Repetition(Repetition),
    /// A possibly capturing group, which contains a child expression.
    Group(Group),
    /// A concatenation of expressions. A concatenation always has at least two
    /// child expressions.
    ///
    /// A concatenation matches only if each of its child expression matches
    /// one after the other.
    Concat(Vec<Hir>),
    /// An alternation of expressions. An alternation always has at least two
    /// child expressions.
    ///
    /// An alternation matches only if at least one of its child expression
    /// matches. If multiple expressions match, then the leftmost is preferred.
    Alternation(Vec<Hir>),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionRange {
    /// Matches a sub-expression exactly this many times.
    Exactly(u32),
    /// Matches a sub-expression at least this many times.
    AtLeast(u32),
    /// Matches a sub-expression at least `m` times and at most `n` times.
    Bounded(u32, u32),
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
    pub fn any_complete(&self) -> bool {
        self.lits.iter().any(|lit| !lit.is_cut())
    }
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
    pub fn cross_add(&mut self, bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return true;
        }
        if self.lits.is_empty() {
            let i = cmp::min(self.limit_size, bytes.len());
            self.lits.push(Literal::new(bytes[..i].to_owned()));
            self.lits[0].cut = i < bytes.len();
            return !self.lits[0].is_cut();
        }
        let size = self.num_bytes();
        if size + self.lits.len() >= self.limit_size {
            return false;
        }
        let mut i = 1;
        while size + (i * self.lits.len()) <= self.limit_size && i < bytes.len() {
            i += 1;
        }
        for lit in &mut self.lits {
            if !lit.is_cut() {
                lit.extend(&bytes[..i]);
                if i < bytes.len() {
                    lit.cut();
                }
            }
        }
        true
    }
    pub fn add(&mut self, lit: Literal) -> bool {
        if self.num_bytes() + lit.len() > self.limit_size {
            return false;
        }
        self.lits.push(lit);
        true
    }
    pub fn add_char_class(&mut self, cls: &hir::ClassUnicode) -> bool {}
    fn add_char_class_reverse(&mut self, cls: &hir::ClassUnicode) -> bool {
        self._add_char_class(cls, true)
    }
    fn _add_char_class(&mut self, cls: &hir::ClassUnicode, reverse: bool) -> bool {}
    pub fn add_byte_class(&mut self, cls: &hir::ClassBytes) -> bool {
        if self.class_exceeds_limits(cls_byte_count(cls)) {
            return false;
        }
        let mut base = self.remove_complete();
        if base.is_empty() {
            base = vec![Literal::empty()];
        }
        for r in cls.iter() {
            let (s, e) = (r.start as u32, r.end as u32 + 1);
            for b in (s..e).map(|b| b as u8) {
                for mut lit in base.clone() {
                    lit.push(b);
                    self.lits.push(lit);
                }
            }
        }
        true
    }
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
impl Literal {
    pub fn new(bytes: Vec<u8>) -> Literal {}
    pub fn empty() -> Literal {
        Literal { v: vec![], cut: false }
    }
    pub fn is_cut(&self) -> bool {}
    pub fn cut(&mut self) {}
}
impl Hir {
    pub fn kind(&self) -> &HirKind {
        &self.kind
    }
    pub fn into_kind(mut self) -> HirKind {}
    pub fn empty() -> Hir {}
    pub fn literal(lit: Literal) -> Hir {}
    pub fn class(class: Class) -> Hir {}
    pub fn anchor(anchor: Anchor) -> Hir {}
    pub fn word_boundary(word_boundary: WordBoundary) -> Hir {}
    pub fn repetition(rep: Repetition) -> Hir {}
    pub fn group(group: Group) -> Hir {}
    pub fn concat(mut exprs: Vec<Hir>) -> Hir {}
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
fn suffixes(expr: &Hir, lits: &mut Literals) {
    match *expr.kind() {
        HirKind::Literal(hir::Literal::Unicode(c)) => {
            let mut buf = [0u8; 4];
            let i = c.encode_utf8(&mut buf).len();
            let mut buf = &mut buf[..i];
            buf.reverse();
            lits.cross_add(buf);
        }
        HirKind::Literal(hir::Literal::Byte(b)) => {
            lits.cross_add(&[b]);
        }
        HirKind::Class(hir::Class::Unicode(ref cls)) => {
            if !lits.add_char_class_reverse(cls) {
                lits.cut();
            }
        }
        HirKind::Class(hir::Class::Bytes(ref cls)) => {
            if !lits.add_byte_class(cls) {
                lits.cut();
            }
        }
        HirKind::Group(hir::Group { ref hir, .. }) => {
            suffixes(&**hir, lits);
        }
        HirKind::Repetition(ref x) => {
            match x.kind {
                hir::RepetitionKind::ZeroOrOne => {
                    repeat_zero_or_one_literals(&x.hir, lits, suffixes);
                }
                hir::RepetitionKind::ZeroOrMore => {
                    repeat_zero_or_more_literals(&x.hir, lits, suffixes);
                }
                hir::RepetitionKind::OneOrMore => {
                    repeat_one_or_more_literals(&x.hir, lits, suffixes);
                }
                hir::RepetitionKind::Range(ref rng) => {
                    let (min, max) = match *rng {
                        hir::RepetitionRange::Exactly(m) => (m, Some(m)),
                        hir::RepetitionRange::AtLeast(m) => (m, None),
                        hir::RepetitionRange::Bounded(m, n) => (m, Some(n)),
                    };
                    repeat_range_literals(&x.hir, min, max, x.greedy, lits, suffixes)
                }
            }
        }
        HirKind::Concat(ref es) if es.is_empty() => {}
        HirKind::Concat(ref es) if es.len() == 1 => suffixes(&es[0], lits),
        HirKind::Concat(ref es) => {
            for e in es.iter().rev() {
                if let HirKind::Anchor(hir::Anchor::EndText) = *e.kind() {
                    if !lits.is_empty() {
                        lits.cut();
                        break;
                    }
                    lits.add(Literal::empty());
                    continue;
                }
                let mut lits2 = lits.to_empty();
                suffixes(e, &mut lits2);
                if !lits.cross_product(&lits2) || !lits2.any_complete() {
                    lits.cut();
                    break;
                }
            }
        }
        HirKind::Alternation(ref es) => {
            alternate_literals(es, lits, suffixes);
        }
        _ => lits.cut(),
    }
}
