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
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalSet<I> {
    ranges: Vec<I>,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter {}
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {
        self.set.negate();
    }
    pub fn union(&mut self, other: &ClassUnicode) {}
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
}
impl<I: Interval> IntervalSet<I> {
    pub fn new<T: IntoIterator<Item = I>>(intervals: T) -> IntervalSet<I> {}
    pub fn push(&mut self, interval: I) {}
    pub fn iter(&self) -> IntervalSetIter<I> {}
    pub fn intervals(&self) -> &[I] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn union(&mut self, other: &IntervalSet<I>) {}
    pub fn intersect(&mut self, other: &IntervalSet<I>) {}
    pub fn difference(&mut self, other: &IntervalSet<I>) {}
    pub fn symmetric_difference(&mut self, other: &IntervalSet<I>) {}
    pub fn negate(&mut self) {
        if self.ranges.is_empty() {
            let (min, max) = (I::Bound::min_value(), I::Bound::max_value());
            self.ranges.push(I::create(min, max));
            return;
        }
        let drain_end = self.ranges.len();
        if self.ranges[0].lower() > I::Bound::min_value() {
            let upper = self.ranges[0].lower().decrement();
            self.ranges.push(I::create(I::Bound::min_value(), upper));
        }
        for i in 1..drain_end {
            let lower = self.ranges[i - 1].upper().increment();
            let upper = self.ranges[i].lower().decrement();
            self.ranges.push(I::create(lower, upper));
        }
        if self.ranges[drain_end - 1].upper() < I::Bound::max_value() {
            let lower = self.ranges[drain_end - 1].upper().increment();
            self.ranges.push(I::create(lower, I::Bound::max_value()));
        }
        self.ranges.drain(..drain_end);
    }
    fn canonicalize(&mut self) {}
    fn is_canonical(&self) -> bool {}
}
