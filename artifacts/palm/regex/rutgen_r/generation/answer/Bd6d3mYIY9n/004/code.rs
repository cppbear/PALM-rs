// Answer 0

#[test]
fn test_is_canonical_empty_ranges() {
    struct Interval {
        ranges: Vec<(char, char)>,
    }

    impl Interval {
        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0] >= pair[1] {
                    return false;
                }
                // Assuming the is_contiguous check is defined elsewhere
                if pair[0].1 >= pair[1].0 {
                    return false;
                }
            }
            true
        }
    }

    let interval = Interval { ranges: Vec::new() };
    assert!(interval.is_canonical());
}

#[test]
fn test_is_canonical_single_range() {
    struct Interval {
        ranges: Vec<(char, char)>,
    }

    impl Interval {
        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0] >= pair[1] {
                    return false;
                }
                if pair[0].1 >= pair[1].0 {
                    return false;
                }
            }
            true
        }
    }

    let interval = Interval { ranges: vec![('a', 'b')] };
    assert!(interval.is_canonical());
}

#[test]
fn test_is_canonical_non_contiguous_ranges() {
    struct Interval {
        ranges: Vec<(char, char)>,
    }

    impl Interval {
        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0] >= pair[1] {
                    return false;
                }
                if pair[0].1 >= pair[1].0 {
                    return false;
                }
            }
            true
        }
    }

    let interval = Interval { ranges: vec![('a', 'b'), ('c', 'd')] };
    assert!(interval.is_canonical());
}

#[test]
fn test_is_canonical_contiguous_ranges() {
    struct Interval {
        ranges: Vec<(char, char)>,
    }

    impl Interval {
        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0] >= pair[1] {
                    return false;
                }
                if pair[0].1 >= pair[1].0 {
                    return false;
                }
            }
            true
        }
    }

    let interval = Interval { ranges: vec![('a', 'b'), ('b', 'c')] };
    assert!(!interval.is_canonical());
}

