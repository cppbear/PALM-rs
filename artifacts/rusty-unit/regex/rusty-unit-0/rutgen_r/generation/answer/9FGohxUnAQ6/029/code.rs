// Answer 0

#[test]
fn test_difference_with_non_overlapping_ranges() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }
    
    struct Interval {
        lower: i32,
        upper: i32,
    }
    
    impl Interval {
        fn new(lower: i32, upper: i32) -> Self {
            Self { lower, upper }
        }
        
        fn lower(&self) -> i32 {
            self.lower
        }
        
        fn upper(&self) -> i32 {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(self.clone()), None);
            }
            if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval::new(self.lower, other.lower)), Some(Interval::new(other.upper, self.upper)));
            }
            if self.lower < other.lower {
                return (Some(Interval::new(self.lower, other.lower)), None);
            }
            if self.upper > other.upper {
                return (None, Some(Interval::new(other.upper, self.upper)));
            }
            (None, None)
        }
    }
    
    impl Clone for Interval {
        fn clone(&self) -> Self {
            Interval::new(self.lower, self.upper)
        }
    }

    let mut set_a = IntervalSet {
        ranges: vec![Interval::new(1, 5), Interval::new(10, 15)],
    };
    
    let set_b = IntervalSet {
        ranges: vec![Interval::new(6, 9)],
    };
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 2);
}

#[test]
fn test_difference_with_complete_overlap() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }
    
    struct Interval {
        lower: i32,
        upper: i32,
    }
    
    impl Interval {
        fn new(lower: i32, upper: i32) -> Self {
            Self { lower, upper }
        }
        
        fn lower(&self) -> i32 {
            self.lower
        }
        
        fn upper(&self) -> i32 {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(self.clone()), None);
            }
            if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval::new(self.lower, other.lower)), Some(Interval::new(other.upper, self.upper)));
            }
            if self.lower < other.lower {
                return (Some(Interval::new(self.lower, other.lower)), None);
            }
            if self.upper > other.upper {
                return (None, Some(Interval::new(other.upper, self.upper)));
            }
            (None, None)
        }
    }

    impl Clone for Interval {
        fn clone(&self) -> Self {
            Interval::new(self.lower, self.upper)
        }
    }
    
    let mut set_a = IntervalSet {
        ranges: vec![Interval::new(2, 6)],
    };
    
    let set_b = IntervalSet {
        ranges: vec![Interval::new(1, 5)],
    };

    set_a.difference(&set_b);
    
    assert!(set_a.ranges.is_empty());
}

#[test]
fn test_difference_with_containment() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }
    
    struct Interval {
        lower: i32,
        upper: i32,
    }
    
    impl Interval {
        fn new(lower: i32, upper: i32) -> Self {
            Self { lower, upper }
        }
        
        fn lower(&self) -> i32 {
            self.lower
        }
        
        fn upper(&self) -> i32 {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(self.clone()), None);
            }
            if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval::new(self.lower, other.lower)), Some(Interval::new(other.upper, self.upper)));
            }
            if self.lower < other.lower {
                return (Some(Interval::new(self.lower, other.lower)), None);
            }
            if self.upper > other.upper {
                return (None, Some(Interval::new(other.upper, self.upper)));
            }
            (None, None)
        }
    }

    impl Clone for Interval {
        fn clone(&self) -> Self {
            Interval::new(self.lower, self.upper)
        }
    }

    let mut set_a = IntervalSet {
        ranges: vec![Interval::new(5, 10)],
    };

    let set_b = IntervalSet {
        ranges: vec![Interval::new(3, 8)],
    };

    set_a.difference(&set_b);
    
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].lower(), 8);
    assert_eq!(set_a.ranges[0].upper(), 10);
}

#[test]
fn test_difference_empty_final_state() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }
    
    struct Interval {
        lower: i32,
        upper: i32,
    }
    
    impl Interval {
        fn new(lower: i32, upper: i32) -> Self {
            Self { lower, upper }
        }
        
        fn lower(&self) -> i32 {
            self.lower
        }
        
        fn upper(&self) -> i32 {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        
        fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            if self.is_intersection_empty(other) {
                return (Some(self.clone()), None);
            }
            if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval::new(self.lower, other.lower)), Some(Interval::new(other.upper, self.upper)));
            }
            if self.lower < other.lower {
                return (Some(Interval::new(self.lower, other.lower)), None);
            }
            if self.upper > other.upper {
                return (None, Some(Interval::new(other.upper, self.upper)));
            }
            (None, None)
        }
    }

    impl Clone for Interval {
        fn clone(&self) -> Self {
            Interval::new(self.lower, self.upper)
        }
    }

    let mut set_a = IntervalSet {
        ranges: vec![Interval::new(1, 5)],
    };

    let set_b = IntervalSet {
        ranges: vec![Interval::new(1, 5)],
    };

    set_a.difference(&set_b);
    
    assert!(set_a.ranges.is_empty());
}

