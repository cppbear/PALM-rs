use std::char;
use std::cmp;
use std::error;
use std::fmt;
use std::u8;
use ast::Span;
use hir::interval::{Interval, IntervalSet, IntervalSetIter};
use unicode;
pub use hir::visitor::{Visitor, visit};
macro_rules! define_bool {
    ($bit:expr, $is_fn_name:ident, $set_fn_name:ident) => {
        fn $is_fn_name (& self) -> bool { self.bools & (0b1 << $bit) > 0 } fn
        $set_fn_name (& mut self, yes : bool) { if yes { self.bools |= 1 << $bit; } else
        { self.bools &= ! (1 << $bit); } }
    };
}
pub trait Interval: Clone + Copy + Debug + Default + Eq + PartialEq + PartialOrd + Ord {
    type Bound: Bound;
    fn lower(&self) -> Self::Bound;
    fn upper(&self) -> Self::Bound;
    fn set_lower(&mut self, bound: Self::Bound);
    fn set_upper(&mut self, bound: Self::Bound);
    fn case_fold_simple(&self, intervals: &mut Vec<Self>);
    fn create(lower: Self::Bound, upper: Self::Bound) -> Self {
        let mut int = Self::default();
        if lower <= upper {
            int.set_lower(lower);
            int.set_upper(upper);
        } else {
            int.set_lower(upper);
            int.set_upper(lower);
        }
        int
    }
    fn union(&self, other: &Self) -> Option<Self> {
        if !self.is_contiguous(other) {
            return None;
        }
        let lower = cmp::min(self.lower(), other.lower());
        let upper = cmp::max(self.upper(), other.upper());
        Some(Self::create(lower, upper))
    }
    fn intersect(&self, other: &Self) -> Option<Self> {
        let lower = cmp::max(self.lower(), other.lower());
        let upper = cmp::min(self.upper(), other.upper());
        if lower <= upper { Some(Self::create(lower, upper)) } else { None }
    }
    fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        if self.is_subset(other) {
            return (None, None);
        }
        if self.is_intersection_empty(other) {
            return (Some(self.clone()), None);
        }
        let add_lower = other.lower() > self.lower();
        let add_upper = other.upper() < self.upper();
        assert!(add_lower || add_upper);
        let mut ret = (None, None);
        if add_lower {
            let upper = other.lower().decrement();
            ret.0 = Some(Self::create(self.lower(), upper));
        }
        if add_upper {
            let lower = other.upper().increment();
            let range = Self::create(lower, self.upper());
            if ret.0.is_none() {
                ret.0 = Some(range);
            } else {
                ret.1 = Some(range);
            }
        }
        ret
    }
    fn symmetric_difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        let union = match self.union(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(union) => union,
        };
        let intersection = match self.intersect(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(intersection) => intersection,
        };
        union.difference(&intersection)
    }
    fn is_contiguous(&self, other: &Self) -> bool;
    fn is_intersection_empty(&self, other: &Self) -> bool;
    fn is_subset(&self, other: &Self) -> bool;
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
#[derive(Debug)]
pub struct SimpleFoldIter(::std::slice::Iter<'static, char>);
impl Interval for ClassUnicodeRange {
    type Bound = char;
    #[inline]
    fn lower(&self) -> char {}
    #[inline]
    fn upper(&self) -> char {}
    #[inline]
    fn set_lower(&mut self, bound: char) {}
    #[inline]
    fn set_upper(&mut self, bound: char) {}
    fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) {
        if !unicode::contains_simple_case_mapping(self.start, self.end) {
            return;
        }
        let start = self.start as u32;
        let end = (self.end as u32).saturating_add(1);
        let mut next_simple_cp = None;
        for cp in (start..end).filter_map(char::from_u32) {
            if next_simple_cp.map_or(false, |next| cp < next) {
                continue;
            }
            let it = match unicode::simple_fold(cp) {
                Ok(it) => it,
                Err(next) => {
                    next_simple_cp = next;
                    continue;
                }
            };
            for cp_folded in it {
                ranges.push(ClassUnicodeRange::new(cp_folded, cp_folded));
            }
        }
    }
}
impl Iterator for SimpleFoldIter {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.0.next().map(|c| *c)
    }
}
impl ClassUnicodeRange {
    pub fn new(start: char, end: char) -> ClassUnicodeRange {
        ClassUnicodeRange::create(start, end)
    }
    pub fn start(&self) -> char {}
    pub fn end(&self) -> char {}
}
pub fn contains_simple_case_mapping(start: char, end: char) -> bool {
    assert!(start <= end);
    CASE_FOLDING_SIMPLE
        .binary_search_by(|&(c, _)| {
            if start <= c && c <= end {
                Ordering::Equal
            } else if c > end {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .is_ok()
}
pub fn simple_fold(c: char) -> result::Result<SimpleFoldIter, Option<char>> {
    CASE_FOLDING_SIMPLE
        .binary_search_by_key(&c, |&(c1, _)| c1)
        .map(|i| SimpleFoldIter(CASE_FOLDING_SIMPLE[i].1.iter()))
        .map_err(|i| {
            if i >= CASE_FOLDING_SIMPLE.len() {
                None
            } else {
                Some(CASE_FOLDING_SIMPLE[i].0)
            }
        })
}
