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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalSet<I> {
    ranges: Vec<I>,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassBytesRange {
    start: u8,
    end: u8,
}
impl ClassBytes {
    pub fn new<I>(ranges: I) -> ClassBytes
    where
        I: IntoIterator<Item = ClassBytesRange>,
    {}
    pub fn empty() -> ClassBytes {}
    pub fn push(&mut self, range: ClassBytesRange) {}
    pub fn iter(&self) -> ClassBytesIter {}
    pub fn ranges(&self) -> &[ClassBytesRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassBytes) {}
    pub fn intersect(&mut self, other: &ClassBytes) {}
    pub fn difference(&mut self, other: &ClassBytes) {
        self.set.difference(&other.set);
    }
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {}
    pub fn is_all_ascii(&self) -> bool {}
}
impl<I: Interval> IntervalSet<I> {
    pub fn new<T: IntoIterator<Item = I>>(intervals: T) -> IntervalSet<I> {}
    pub fn push(&mut self, interval: I) {}
    pub fn iter(&self) -> IntervalSetIter<I> {}
    pub fn intervals(&self) -> &[I] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn union(&mut self, other: &IntervalSet<I>) {}
    pub fn intersect(&mut self, other: &IntervalSet<I>) {}
    pub fn difference(&mut self, other: &IntervalSet<I>) {
        if self.ranges.is_empty() || other.ranges.is_empty() {
            return;
        }
        let drain_end = self.ranges.len();
        let (mut a, mut b) = (0, 0);
        'LOOP: while a < drain_end && b < other.ranges.len() {
            if other.ranges[b].upper() < self.ranges[a].lower() {
                b += 1;
                continue;
            }
            if self.ranges[a].upper() < other.ranges[b].lower() {
                let range = self.ranges[a];
                self.ranges.push(range);
                a += 1;
                continue;
            }
            assert!(! self.ranges[a].is_intersection_empty(& other.ranges[b]));
            let mut range = self.ranges[a];
            while b < other.ranges.len()
                && !range.is_intersection_empty(&other.ranges[b])
            {
                let old_range = range;
                range = match range.difference(&other.ranges[b]) {
                    (None, None) => {
                        a += 1;
                        continue 'LOOP;
                    }
                    (Some(range1), None) | (None, Some(range1)) => range1,
                    (Some(range1), Some(range2)) => {
                        self.ranges.push(range1);
                        range2
                    }
                };
                if other.ranges[b].upper() > old_range.upper() {
                    break;
                }
                b += 1;
            }
            self.ranges.push(range);
            a += 1;
        }
        while a < drain_end {
            let range = self.ranges[a];
            self.ranges.push(range);
            a += 1;
        }
        self.ranges.drain(..drain_end);
    }
    pub fn symmetric_difference(&mut self, other: &IntervalSet<I>) {}
    pub fn negate(&mut self) {}
    fn canonicalize(&mut self) {}
    fn is_canonical(&self) -> bool {}
}
