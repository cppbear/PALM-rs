// Answer 0

#[test]
fn test_union_empty_sets() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        pub fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }

        pub fn union(&mut self, other: &IntervalSet<I>) {
            self.ranges.extend(&other.ranges);
            self.canonicalize();
        }

        fn canonicalize(&mut self) {
            // Dummy implementation for test purposes
            self.ranges.sort(); // Just to simulate a canonicalization
        }
    }

    let mut set1: IntervalSet<u32> = IntervalSet::new();
    let set2: IntervalSet<u32> = IntervalSet::new();
    
    set1.union(&set2);

    assert_eq!(set1.ranges.len(), 0);
}

#[test]
fn test_union_non_empty_sets() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        pub fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }

        pub fn union(&mut self, other: &IntervalSet<I>) {
            self.ranges.extend(&other.ranges);
            self.canonicalize();
        }

        fn canonicalize(&mut self) {
            self.ranges.sort(); // Simulating a canonicalization
        }
    }

    let mut set1 = IntervalSet { ranges: vec![1, 2, 3] };
    let set2 = IntervalSet { ranges: vec![4, 5] };
    
    set1.union(&set2);

    assert_eq!(set1.ranges, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_union_with_overlap() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        pub fn new() -> Self {
            IntervalSet { ranges: Vec::new() }
        }

        pub fn union(&mut self, other: &IntervalSet<I>) {
            self.ranges.extend(&other.ranges);
            self.canonicalize();
        }

        fn canonicalize(&mut self) {
            // Dummy logic to merge overlapping ranges for testing
            self.ranges.sort(); // Sort ranges
            self.ranges.dedup(); // Remove duplicates
        }
    }

    let mut set1 = IntervalSet { ranges: vec![1, 2, 3, 5] };
    let set2 = IntervalSet { ranges: vec![3, 4, 5] };
    
    set1.union(&set2);

    assert_eq!(set1.ranges, vec![1, 2, 3, 4, 5]);
}

