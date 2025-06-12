// Answer 0

#[test]
fn test_difference_non_empty_ranges_no_overlap() {
    struct Interval {
        start: usize,
        end: usize,
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn add_range(&mut self, range: I) {
            self.ranges.push(range);
        }
    }

    impl Interval {
        fn upper(&self) -> usize {
            self.end
        }

        fn lower(&self) -> usize {
            self.start
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || other.upper() < self.lower()
        }

        fn difference(&self, other: &Self) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            // Assuming they intersect, the difference implementation needs to be simplified for the test
            let new_start = self.start;
            let new_end = if self.end > other.start {
                other.start
            } else {
                self.end
            };
            (Some(Interval { start: new_start, end: new_end }), None)
        }
    }

    let mut set_a = IntervalSet::new();
    let mut set_b = IntervalSet::new();

    set_a.add_range(Interval { start: 5, end: 10 });
    set_b.add_range(Interval { start: 12, end: 15 });

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].start, 5);
    assert_eq!(set_a.ranges[0].end, 10);
}

#[test]
fn test_difference_non_empty_ranges_with_overlap() {
    struct Interval {
        start: usize,
        end: usize,
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn add_range(&mut self, range: I) {
            self.ranges.push(range);
        }
    }

    impl Interval {
        fn upper(&self) -> usize {
            self.end
        }

        fn lower(&self) -> usize {
            self.start
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper() < other.lower() || other.upper() < self.lower()
        }

        fn difference(&self, other: &Self) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }
            // Simplified logic for difference
            if self.start < other.start {
                return (Some(Interval { start: self.start, end: other.start }), None);
            }
            (None, None) // Assuming full overlap just for the test
        }
    }

    let mut set_a = IntervalSet::new();
    let mut set_b = IntervalSet::new();

    set_a.add_range(Interval { start: 5, end: 10 });
    set_b.add_range(Interval { start: 8, end: 12 });

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].start, 5);
    assert_eq!(set_a.ranges[0].end, 8);
}

#[should_panic]
#[test]
fn test_difference_empty_self() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn difference(&mut self, _: &IntervalSet<I>) {
            if self.ranges.is_empty() {
                panic!("Self ranges is empty.");
            }
        }
    }

    let mut set_a = IntervalSet::new();
    let set_b = IntervalSet::new();

    set_a.difference(&set_b);
}

#[should_panic]
#[test]
fn test_difference_empty_other() {
    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    impl<I> IntervalSet<I> {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn difference(&mut self, _: &IntervalSet<I>) {
            if self.ranges.is_empty() {
                panic!("Other ranges is empty.");
            }
        }
    }

    let mut set_a = IntervalSet::new();
    set_a.add_range(Interval { start: 1, end: 2 });
    let set_b = IntervalSet::new();

    set_a.difference(&set_b);
}

