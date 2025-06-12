// Answer 0

#[derive(Debug)]
struct IntervalSet {
    ranges: Vec<(u32, u32)>,
}

impl IntervalSet {
    pub fn new() -> Self {
        IntervalSet {
            ranges: Vec::new(),
        }
    }
    
    pub fn insert(&mut self, start: u32, end: u32) {
        self.ranges.push((start, end));
        self.ranges.sort(); // Sorting to maintain ascending order for the iterator
    }

    pub fn iter(&self) -> IntervalSetIter {
        IntervalSetIter(self.ranges.iter())
    }
}

pub struct IntervalSetIter<'a>(std::slice::Iter<'a, (u32, u32)>);

impl<'a> Iterator for IntervalSetIter<'a> {
    type Item = &'a (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[test]
fn test_empty_interval_set() {
    let interval_set = IntervalSet::new();
    let mut iter = interval_set.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_single_interval() {
    let mut interval_set = IntervalSet::new();
    interval_set.insert(1, 5);
    let mut iter = interval_set.iter();
    assert_eq!(iter.next(), Some(&(1, 5)));
    assert!(iter.next().is_none());
}

#[test]
fn test_multiple_intervals() {
    let mut interval_set = IntervalSet::new();
    interval_set.insert(3, 4);
    interval_set.insert(1, 2);
    interval_set.insert(5, 6);
    
    let mut iter = interval_set.iter();
    assert_eq!(iter.next(), Some(&(1, 2)));
    assert_eq!(iter.next(), Some(&(3, 4)));
    assert_eq!(iter.next(), Some(&(5, 6)));
    assert!(iter.next().is_none());
}

#[test]
fn test_overlapping_intervals() {
    let mut interval_set = IntervalSet::new();
    interval_set.insert(1, 5);
    interval_set.insert(2, 3);
    
    let mut iter = interval_set.iter();
    assert_eq!(iter.next(), Some(&(1, 5))); // The larger interval should be yielded first in this case
    assert!(iter.next().is_none());
}

