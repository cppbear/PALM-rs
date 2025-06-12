// Answer 0

#[derive(Debug)]
struct IntervalSet<I> {
    ranges: Vec<I>,
}

impl<I> IntervalSet<I> {
    pub fn new() -> Self {
        IntervalSet { ranges: Vec::new() }
    }
    
    pub fn intervals(&self) -> &[I] {
        &self.ranges
    }
    
    pub fn add_range(&mut self, range: I) {
        self.ranges.push(range);
    }
}

#[test]
fn test_intervals_empty() {
    let set: IntervalSet<u32> = IntervalSet::new();
    assert_eq!(set.intervals(), &[]);
}

#[test]
fn test_intervals_single() {
    let mut set = IntervalSet::new();
    set.add_range(1);
    assert_eq!(set.intervals(), &[1]);
}

#[test]
fn test_intervals_multiple() {
    let mut set = IntervalSet::new();
    set.add_range(1);
    set.add_range(2);
    set.add_range(3);
    assert_eq!(set.intervals(), &[1, 2, 3]);
}

#[test]
fn test_intervals_boundary() {
    let mut set = IntervalSet::new();
    set.add_range(0);
    set.add_range(u32::MAX);
    assert_eq!(set.intervals(), &[0, u32::MAX]);
}

