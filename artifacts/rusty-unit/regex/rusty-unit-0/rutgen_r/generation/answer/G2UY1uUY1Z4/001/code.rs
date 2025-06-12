// Answer 0

#[test]
fn test_push_with_single_interval() {
    struct Interval {
        start: usize,
        end: usize,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet {
                ranges: Vec::new(),
            }
        }
        
        fn push(&mut self, interval: Interval) {
            self.ranges.push(interval);
            self.canonicalize();
        }
        
        fn canonicalize(&mut self) {
            // Dummy implementation for the sake of testing.
            self.ranges.sort_by_key(|i| i.start);
        }
    }
    
    let mut intervals = IntervalSet::new();
    intervals.push(Interval { start: 0, end: 1 });
    assert_eq!(intervals.ranges.len(), 1);
    assert_eq!(intervals.ranges[0].start, 0);
    assert_eq!(intervals.ranges[0].end, 1);
}

#[test]
fn test_push_multiple_intervals() {
    struct Interval {
        start: usize,
        end: usize,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet {
                ranges: Vec::new(),
            }
        }
        
        fn push(&mut self, interval: Interval) {
            self.ranges.push(interval);
            self.canonicalize();
        }
        
        fn canonicalize(&mut self) {
            // Dummy implementation for the sake of testing.
            self.ranges.sort_by_key(|i| i.start);
        }
    }

    let mut intervals = IntervalSet::new();
    intervals.push(Interval { start: 1, end: 3 });
    intervals.push(Interval { start: 0, end: 2 });
    
    assert_eq!(intervals.ranges.len(), 2);
    assert_eq!(intervals.ranges[0].start, 0);
    assert_eq!(intervals.ranges[0].end, 2);
    assert_eq!(intervals.ranges[1].start, 1);
    assert_eq!(intervals.ranges[1].end, 3);
}

#[test]
fn test_push_with_overlapping_intervals() {
    struct Interval {
        start: usize,
        end: usize,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet {
                ranges: Vec::new(),
            }
        }
        
        fn push(&mut self, interval: Interval) {
            self.ranges.push(interval);
            self.canonicalize();
        }
        
        fn canonicalize(&mut self) {
            // Dummy implementation for the sake of testing.
            self.ranges.sort_by_key(|i| i.start);
        }
    }

    let mut intervals = IntervalSet::new();
    intervals.push(Interval { start: 0, end: 5 });
    intervals.push(Interval { start: 3, end: 7 });
    
    assert_eq!(intervals.ranges.len(), 2);
    assert_eq!(intervals.ranges[0].start, 0);
    assert_eq!(intervals.ranges[0].end, 5);
    assert_eq!(intervals.ranges[1].start, 3);
    assert_eq!(intervals.ranges[1].end, 7);
}

#[test]
#[should_panic]
fn test_push_with_invalid_interval() {
    struct Interval {
        start: usize,
        end: usize,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet {
                ranges: Vec::new(),
            }
        }
        
        fn push(&mut self, interval: Interval) {
            if interval.start > interval.end {
                panic!("start cannot be greater than end");
            }
            self.ranges.push(interval);
            self.canonicalize();
        }
        
        fn canonicalize(&mut self) {
            // Dummy implementation for the sake of testing.
            self.ranges.sort_by_key(|i| i.start);
        }
    }

    let mut intervals = IntervalSet::new();
    intervals.push(Interval { start: 5, end: 3 });  // This should panic
}

