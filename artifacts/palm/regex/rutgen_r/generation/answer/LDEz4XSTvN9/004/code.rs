// Answer 0

#[derive(Debug)]
struct Bound {
    value: i32,
}

impl Bound {
    fn min_value() -> Self {
        Bound { value: i32::MIN }
    }
    
    fn max_value() -> Self {
        Bound { value: i32::MAX }
    }
    
    fn increment(&self) -> Self {
        Bound { value: self.value + 1 }
    }
    
    fn decrement(&self) -> Self {
        Bound { value: self.value - 1 }
    }
}

#[derive(Debug)]
struct Range {
    lower: Bound,
    upper: Bound,
}

impl Range {
    fn new(lower: Bound, upper: Bound) -> Self {
        Range { lower, upper }
    }
    
    fn lower(&self) -> &Bound {
        &self.lower
    }
    
    fn upper(&self) -> &Bound {
        &self.upper
    }
}

#[derive(Debug)]
struct IntervalSet {
    ranges: Vec<Range>,
}

impl IntervalSet {
    fn new() -> Self {
        IntervalSet { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }
    
    pub fn negate(&mut self) {
        if self.ranges.is_empty() {
            let (min, max) = (Bound::min_value(), Bound::max_value());
            self.ranges.push(Range::new(min, max));
            return;
        }

        let drain_end = self.ranges.len();

        if self.ranges[0].lower().value > Bound::min_value().value {
            let upper = self.ranges[0].lower().decrement();
            self.ranges.push(Range::new(Bound::min_value(), upper));
        }
        for i in 1..drain_end {
            let lower = self.ranges[i - 1].upper().increment();
            let upper = self.ranges[i].lower().decrement();
            self.ranges.push(Range::new(lower, upper));
        }
        if self.ranges[drain_end - 1].upper().value < Bound::max_value().value {
            let lower = self.ranges[drain_end - 1].upper().increment();
            self.ranges.push(Range::new(lower, Bound::max_value()));
        }
        self.ranges.drain(..drain_end);
    }
}

#[test]
fn test_negate_empty_case() {
    let mut interval_set = IntervalSet::new();
    interval_set.negate();
    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0].lower.value, i32::MIN);
    assert_eq!(interval_set.ranges[0].upper.value, i32::MAX);
}

#[test]
fn test_negate_with_single_range() {
    let mut interval_set = IntervalSet::new();
    interval_set.add_range(Range::new(Bound { value: 0 }, Bound { value: 10 }));
    interval_set.negate();
    assert_eq!(interval_set.ranges.len(), 3);
    assert_eq!(interval_set.ranges[0].lower.value, i32::MIN);
    assert_eq!(interval_set.ranges[0].upper.value, -1);
    assert_eq!(interval_set.ranges[1].lower.value, 11);
    assert_eq!(interval_set.ranges[1].upper.value, i32::MAX);
}

#[test]
fn test_negate_with_multiple_ranges() {
    let mut interval_set = IntervalSet::new();
    interval_set.add_range(Range::new(Bound { value: 0 }, Bound { value: 5 }));
    interval_set.add_range(Range::new(Bound { value: 10 }, Bound { value: 15 }));
    interval_set.negate();
    assert_eq!(interval_set.ranges.len(), 4);
    assert_eq!(interval_set.ranges[0].lower.value, i32::MIN);
    assert_eq!(interval_set.ranges[0].upper.value, 4);
    assert_eq!(interval_set.ranges[1].lower.value, 6);
    assert_eq!(interval_set.ranges[1].upper.value, 9);
    assert_eq!(interval_set.ranges[2].lower.value, 16);
    assert_eq!(interval_set.ranges[2].upper.value, i32::MAX);
}

