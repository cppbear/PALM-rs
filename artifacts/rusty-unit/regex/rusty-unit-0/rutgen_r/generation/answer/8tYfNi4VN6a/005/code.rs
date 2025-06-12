// Answer 0

#[test]
fn test_canonicalize_non_canonical_non_empty_ranges() {
    struct RangeSet {
        ranges: Vec<(usize, usize)>,
    }

    impl RangeSet {
        fn is_canonical(&self) -> bool {
            // For testing, we define that it is not canonical if it has overlapping ranges
            false
        }

        fn canonicalize(&mut self) {
            if self.is_canonical() {
                return;
            }
            self.ranges.sort();
            assert!(!self.ranges.is_empty());

            let drain_end = self.ranges.len();
            for oldi in 0..drain_end {
                if self.ranges.len() > drain_end {
                    let (last, rest) = self.ranges.split_last_mut().unwrap();
                    if let Some(union) = last.1.checked_add(rest[oldi].0) { // mock union operation
                        *last = (*last.0, union); // simulates a successful union
                        continue;
                    }
                }
                let range = self.ranges[oldi];
                self.ranges.push(range);
            }
            self.ranges.drain(..drain_end);
        }
    }

    let mut set = RangeSet {
        ranges: vec![(1, 2), (3, 4)],
    };

    set.canonicalize();

    assert_eq!(set.ranges, vec![(1, 2), (3, 4)]);
}

#[test]
#[should_panic]
fn test_canonicalize_should_panic_on_empty_ranges() {
    struct RangeSet {
        ranges: Vec<(usize, usize)>,
    }

    impl RangeSet {
        fn is_canonical(&self) -> bool {
            false
        }

        fn canonicalize(&mut self) {
            if self.is_canonical() {
                return;
            }
            self.ranges.sort();
            assert!(!self.ranges.is_empty());

            let drain_end = self.ranges.len();
            for oldi in 0..drain_end {
                if self.ranges.len() > drain_end {
                    let (last, rest) = self.ranges.split_last_mut().unwrap();
                    if let Some(union) = last.1.checked_add(rest[oldi].0) {
                        *last = (*last.0, union);
                        continue;
                    }
                }
                let range = self.ranges[oldi];
                self.ranges.push(range);
            }
            self.ranges.drain(..drain_end);
        }
    }

    let mut set = RangeSet {
        ranges: Vec::new(),
    };

    set.canonicalize();
}

