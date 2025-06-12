// Answer 0

#[test]
fn test_difference_non_overlapping_ranges() {
    struct Interval {
        lower: usize,
        upper: usize,
    }
    
    impl Interval {
        fn lower(&self) -> usize {
            self.lower
        }
        
        fn upper(&self) -> usize {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }
        
        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            if self.lower < other.lower {
                if self.upper > other.upper {
                    return (Some(Interval { lower: self.lower, upper: other.lower }), Some(Interval { lower: other.upper + 1, upper: self.upper }));
                } else {
                    return (Some(Interval { lower: self.lower, upper: other.lower }), None);
                }
            } else if self.upper > other.upper {
                return (None, Some(Interval { lower: other.upper + 1, upper: self.upper }));
            }
            (None, None)
        }
    }
    
    struct IntervalSet {
        ranges: Vec<Interval>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }
        
        fn add_range(&mut self, range: Interval) {
            self.ranges.push(range);
        }
        
        fn difference(&mut self, other: &IntervalSet) {
            // original difference function goes here
        }
    }
    
    // Setup
    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval { lower: 1, upper: 5 });
    set_a.add_range(Interval { lower: 6, upper: 10 });
    
    let mut set_b = IntervalSet::new();
    set_b.add_range(Interval { lower: 11, upper: 15 });
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0].lower, 1);
    assert_eq!(set_a.ranges[0].upper, 5);
    assert_eq!(set_a.ranges[1].lower, 6);
    assert_eq!(set_a.ranges[1].upper, 10);
}

#[test]
fn test_difference_overlap_removal() {
    struct Interval {
        lower: usize,
        upper: usize,
    }
    
    impl Interval {
        fn lower(&self) -> usize {
            self.lower
        }
        
        fn upper(&self) -> usize {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }
        
        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            if self.lower < other.lower {
                if self.upper > other.upper {
                    return (Some(Interval { lower: self.lower, upper: other.lower }), Some(Interval { lower: other.upper + 1, upper: self.upper }));
                } else {
                    return (Some(Interval { lower: self.lower, upper: other.lower }), None);
                }
            } else if self.upper > other.upper {
                return (None, Some(Interval { lower: other.upper + 1, upper: self.upper }));
            }
            (None, None)
        }
    }
    
    struct IntervalSet {
        ranges: Vec<Interval>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }
        
        fn add_range(&mut self, range: Interval) {
            self.ranges.push(range);
        }
        
        fn difference(&mut self, other: &IntervalSet) {
            // original difference function goes here
        }
    }
    
    // Setup
    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval { lower: 1, upper: 5 });
    
    let mut set_b = IntervalSet::new();
    set_b.add_range(Interval { lower: 3, upper: 4 });
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].lower, 1);
    assert_eq!(set_a.ranges[0].upper, 2);
}

#[test]
#[should_panic]
fn test_difference_panic_condition() {
    struct Interval {
        lower: usize,
        upper: usize,
    }
    
    impl Interval {
        fn lower(&self) -> usize {
            self.lower
        }
        
        fn upper(&self) -> usize {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }
        
        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            (None, None) // Simulate full overlap for panic test
        }
    }
    
    struct IntervalSet {
        ranges: Vec<Interval>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }
        
        fn add_range(&mut self, range: Interval) {
            self.ranges.push(range);
        }
        
        fn difference(&mut self, other: &IntervalSet) {
            // original difference function goes here
        }
    }
    
    // Panic-inducing scenario setup
    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval { lower: 1, upper: 5 });
    
    let mut set_b = IntervalSet::new();
    set_b.add_range(Interval { lower: 1, upper: 5 });
    
    set_a.difference(&set_b); // This should panic
}

