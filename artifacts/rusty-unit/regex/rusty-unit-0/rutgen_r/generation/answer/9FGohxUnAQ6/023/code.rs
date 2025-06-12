// Answer 0

#[test]
fn test_difference_non_empty_ranges() {
    struct Range {
        lower: usize,
        upper: usize,
    }

    impl Range {
        fn lower(&self) -> usize {
            self.lower
        }
        
        fn upper(&self) -> usize {
            self.upper
        }
        
        fn is_intersection_empty(&self, other: &Range) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn difference(&self, other: &Range) -> (Option<Range>, Option<Range>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            }

            if self.lower == other.lower && self.upper == other.upper {
                return (None, None);
            }

            let lower_part = if self.lower < other.lower {
                Some(Range { lower: self.lower, upper: other.lower })
            } else {
                None
            };

            let upper_part = if self.upper > other.upper {
                Some(Range { lower: other.upper, upper: self.upper })
            } else {
                None
            };

            (lower_part, upper_part)
        }
    }

    struct IntervalSet {
        ranges: Vec<Range>,
    }

    impl IntervalSet {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }
        
        fn push(&mut self, range: Range) {
            self.ranges.push(range);
        }

        pub fn difference(&mut self, other: &IntervalSet) {
            if self.ranges.is_empty() || other.ranges.is_empty() {
                return;
            }

            let drain_end = self.ranges.len();
            let (mut a, mut b) = (0, 0);
        'LOOP:
            while a < drain_end && b < other.ranges.len() {
                if other.ranges[b].upper() < self.ranges[a].lower() {
                    b += 1;
                    continue;
                }
                
                if self.ranges[a].upper() < other.ranges[b].lower() {
                    let range = self.ranges[a];
                    self.ranges.push(range);
                    a += 1;
                    continue;
                }

                assert!(!self.ranges[a].is_intersection_empty(&other.ranges[b]));
                
                let mut range = self.ranges[a];
                while b < other.ranges.len() && !range.is_intersection_empty(&other.ranges[b]) {
                    let old_range = range;
                    range = match range.difference(&other.ranges[b]) {
                        (None, None) => {
                            a += 1;
                            continue 'LOOP;
                        },
                        (Some(range1), None) | (None, Some(range1)) => range1,
                        (Some(range1), Some(range2)) => {
                            self.ranges.push(range1);
                            range2
                        }
                    };

                    if other.ranges[b].upper() > old_range.upper() {
                        break;
                    }
                    b += 1;
                }
                self.ranges.push(range);
                a += 1;
            }
            while a < drain_end {
                let range = self.ranges[a];
                self.ranges.push(range);
                a += 1;
            }
            self.ranges.drain(..drain_end);
        }
    }

    let mut set_a = IntervalSet::new();
    let range_1_a = Range { lower: 1, upper: 5 };
    let range_2_a = Range { lower: 6, upper: 10 };
    let range_3_a = Range { lower: 11, upper: 15 };
    set_a.push(range_1_a);
    set_a.push(range_2_a);
    set_a.push(range_3_a);

    let mut set_b = IntervalSet::new();
    let range_1_b = Range { lower: 4, upper: 12 };
    set_b.push(range_1_b);

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 3); // Ranges 1, 6-10, and 11-15 should remain
}

