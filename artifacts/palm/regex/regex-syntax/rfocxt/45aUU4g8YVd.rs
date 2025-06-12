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
