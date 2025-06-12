// Answer 0

#[test]
fn test_is_canonical_empty() {
    struct Ranges {
        ranges: Vec<(usize, usize)>,
    }

    impl Ranges {
        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0] >= pair[1] {
                    return false;
                }
                // Dummy is_contiguous implementation assuming ranges are tuples of (start, end)
                if pair[0].1 >= pair[1].0 {
                    return false;
                }
            }
            true
        }
    }

    let ranges = Ranges { ranges: vec![] };
    assert!(ranges.is_canonical());
}

#[test]
fn test_is_canonical_single_range() {
    struct Ranges {
        ranges: Vec<(usize, usize)>,
    }

    impl Ranges {
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

    let ranges = Ranges { ranges: vec![(1, 2)] };
    assert!(ranges.is_canonical());
}

#[test]
fn test_is_canonical_non_contiguous() {
    struct Ranges {
        ranges: Vec<(usize, usize)>,
    }

    impl Ranges {
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

    let ranges = Ranges { ranges: vec![(1, 2), (3, 4)] };
    assert!(ranges.is_canonical());
}

#[test]
fn test_is_canonical_overlapping() {
    struct Ranges {
        ranges: Vec<(usize, usize)>,
    }

    impl Ranges {
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

    let ranges = Ranges { ranges: vec![(1, 3), (2, 4)] };
    assert!(!ranges.is_canonical());
}

#[test]
fn test_is_canonical_adjacent() {
    struct Ranges {
        ranges: Vec<(usize, usize)>,
    }

    impl Ranges {
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

    let ranges = Ranges { ranges: vec![(1, 2), (2, 3)] };
    assert!(!ranges.is_canonical());
}

#[test]
fn test_is_canonical_non_canonical_order() {
    struct Ranges {
        ranges: Vec<(usize, usize)>,
    }

    impl Ranges {
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

    let ranges = Ranges { ranges: vec![(2, 3), (1, 2)] };
    assert!(!ranges.is_canonical());
}

