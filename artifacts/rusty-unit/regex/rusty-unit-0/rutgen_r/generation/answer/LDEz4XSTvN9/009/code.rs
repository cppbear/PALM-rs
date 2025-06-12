// Answer 0

#[derive(Debug)]
struct Interval {
    lower: i32,
    upper: i32,
}

impl Interval {
    fn create(lower: i32, upper: i32) -> Self {
        Interval { lower, upper }
    }
    
    fn lower(&self) -> i32 {
        self.lower
    }
    
    fn upper(&self) -> i32 {
        self.upper
    }
    
    fn increment(&self) -> i32 {
        self.upper + 1
    }
    
    fn decrement(&self) -> i32 {
        self.lower - 1
    }
}

struct IntervalSet {
    ranges: Vec<Interval>,
}

impl IntervalSet {
    fn new() -> Self {
        IntervalSet { ranges: Vec::new() }
    }
    
    fn negate(&mut self) {
        if self.ranges.is_empty() {
            let (min, max) = (i32::MIN, i32::MAX);
            self.ranges.push(Interval::create(min, max));
            return;
        }
        
        let drain_end = self.ranges.len();

        if self.ranges[0].lower() > i32::MIN {
            let upper = self.ranges[0].lower().decrement();
            self.ranges.push(Interval::create(i32::MIN, upper));
        }
        for i in 1..drain_end {
            let lower = self.ranges[i - 1].upper().increment();
            let upper = self.ranges[i].lower().decrement();
            self.ranges.push(Interval::create(lower, upper));
        }
        if self.ranges[drain_end - 1].upper() < i32::MAX {
            let lower = self.ranges[drain_end - 1].upper().increment();
            self.ranges.push(Interval::create(lower, i32::MAX));
        }
        self.ranges.drain(..drain_end);
    }
}

#[test]
fn test_negate_empty_set() {
    let mut set = IntervalSet::new();
    set.negate();
    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0].lower(), i32::MIN);
    assert_eq!(set.ranges[0].upper(), i32::MAX);
}

#[test]
fn test_negate_single_range() {
    let mut set = IntervalSet { ranges: vec![Interval::create(1, 5)] };
    set.negate();
    assert_eq!(set.ranges.len(), 2);
    assert_eq!(set.ranges[0].lower(), i32::MIN);
    assert_eq!(set.ranges[0].upper(), 0);
    assert_eq!(set.ranges[1].lower(), 6);
    assert_eq!(set.ranges[1].upper(), i32::MAX);
}

#[test]
fn test_negate_multiple_ranges() {
    let mut set = IntervalSet { ranges: vec![Interval::create(1, 3), Interval::create(5, 7)] };
    set.negate();
    assert_eq!(set.ranges.len(), 3);
    assert_eq!(set.ranges[0].lower(), i32::MIN);
    assert_eq!(set.ranges[0].upper(), 0);
    assert_eq!(set.ranges[1].lower(), 4);
    assert_eq!(set.ranges[1].upper(), 4);
    assert_eq!(set.ranges[2].lower(), 8);
    assert_eq!(set.ranges[2].upper(), i32::MAX);
}

#[test]
fn test_negate_boundary_conditions() {
    let mut set = IntervalSet { ranges: vec![Interval::create(i32::MIN + 1, 5), Interval::create(6, i32::MAX - 1)] };
    set.negate();
    assert_eq!(set.ranges.len(), 4);
    assert_eq!(set.ranges[0].lower(), i32::MIN);
    assert_eq!(set.ranges[0].upper(), i32::MIN);
    assert_eq!(set.ranges[1].lower(), 5);
    assert_eq!(set.ranges[1].upper(), 5);
    assert_eq!(set.ranges[2].lower(), 7);
    assert_eq!(set.ranges[2].upper(), i32::MAX - 1);
    assert_eq!(set.ranges[3].lower(), i32::MAX);
    assert_eq!(set.ranges[3].upper(), i32::MAX);
}

