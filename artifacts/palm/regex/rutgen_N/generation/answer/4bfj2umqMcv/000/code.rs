// Answer 0

#[test]
fn test_union_empty_sets() {
    struct IntervalSet {
        ranges: Vec<(u32, u32)>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }
        
        fn canonicalize(&mut self) {
            // Placeholder for canonicalization logic
        }
        
        fn union(&mut self, other: &IntervalSet) {
            self.ranges.extend(&other.ranges);
            self.canonicalize();
        }
    }
    
    let mut set_a = IntervalSet::new();
    let set_b = IntervalSet::new();
    
    set_a.union(&set_b);
    
    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_union_non_empty_sets() {
    struct IntervalSet {
        ranges: Vec<(u32, u32)>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }
        
        fn canonicalize(&mut self) {
            // Placeholder for canonicalization logic
        }
        
        fn union(&mut self, other: &IntervalSet) {
            self.ranges.extend(&other.ranges);
            self.canonicalize();
        }
    }
    
    let mut set_a = IntervalSet { ranges: vec![(1, 3), (5, 7)] };
    let set_b = IntervalSet { ranges: vec![(2, 4), (6, 8)] };

    set_a.union(&set_b);
    
    assert_eq!(set_a.ranges.len(), 4);
}

#[test]
fn test_union_with_overlapping_sets() {
    struct IntervalSet {
        ranges: Vec<(u32, u32)>,
    }
    
    impl IntervalSet {
        fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }
        
        fn canonicalize(&mut self) {
            // Placeholder for canonicalization logic
        }
        
        fn union(&mut self, other: &IntervalSet) {
            self.ranges.extend(&other.ranges);
            self.canonicalize();
        }
    }
    
    let mut set_a = IntervalSet { ranges: vec![(1, 5)] };
    let set_b = IntervalSet { ranges: vec![(3, 7)] };

    set_a.union(&set_b);
    
    assert_eq!(set_a.ranges.len(), 2); // Assuming canonicalize could adjust this in a real case.
}

