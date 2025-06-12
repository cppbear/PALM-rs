use std::char;
use std::cmp;
use std::fmt::Debug;
use std::slice;
use std::u8;
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalSet<I> {
    ranges: Vec<I>,
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
    pub fn negate(&mut self) {}
    fn canonicalize(&mut self) {}
    fn is_canonical(&self) -> bool {
        for pair in self.ranges.windows(2) {
            if pair[0] >= pair[1] {
                return false;
            }
            if pair[0].is_contiguous(&pair[1]) {
                return false;
            }
        }
        true
    }
}
