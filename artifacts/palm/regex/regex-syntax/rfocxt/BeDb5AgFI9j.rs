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
    pub fn case_fold_simple(&mut self) {
        self.set.case_fold_simple();
    }
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassBytes) {}
    pub fn intersect(&mut self, other: &ClassBytes) {}
    pub fn difference(&mut self, other: &ClassBytes) {}
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {}
    pub fn is_all_ascii(&self) -> bool {}
}
impl<I: Interval> IntervalSet<I> {
    pub fn new<T: IntoIterator<Item = I>>(intervals: T) -> IntervalSet<I> {}
    pub fn push(&mut self, interval: I) {}
    pub fn iter(&self) -> IntervalSetIter<I> {}
    pub fn intervals(&self) -> &[I] {}
    pub fn case_fold_simple(&mut self) {
        let len = self.ranges.len();
        for i in 0..len {
            let range = self.ranges[i];
            range.case_fold_simple(&mut self.ranges);
        }
        self.canonicalize();
    }
    pub fn union(&mut self, other: &IntervalSet<I>) {}
    pub fn intersect(&mut self, other: &IntervalSet<I>) {}
    pub fn difference(&mut self, other: &IntervalSet<I>) {}
    pub fn symmetric_difference(&mut self, other: &IntervalSet<I>) {}
    pub fn negate(&mut self) {}
    fn canonicalize(&mut self) {}
    fn is_canonical(&self) -> bool {}
}
