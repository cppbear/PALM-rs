// Answer 0

fn test_is_canonical_non_strictly_increasing() {
    struct Range {
        start: u32,
        end: u32,
    }

    impl Range {
        fn is_contiguous(&self, other: &Range) -> bool {
            self.end + 1 == other.start // consider contiguous as end+1 == start
        }
    }

    struct CanonicalChecker {
        ranges: Vec<Range>,
    }

    impl CanonicalChecker {
        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0].start >= pair[1].start {
                    return false;
                }
                if pair[0].is_contiguous(&pair[1]) {
                    return false;
                }
            }
            true
        }
    }

    // Test case where ranges are not strictly increasing, should return false
    let ranges = vec![
        Range { start: 1, end: 2 },
        Range { start: 2, end: 3 }, // non-contiguous but equal 
    ];
    
    let checker = CanonicalChecker { ranges };
    assert!(!checker.is_canonical());
}

fn test_is_canonical_equal_ranges() {
    struct Range {
        start: u32,
        end: u32,
    }

    impl Range {
        fn is_contiguous(&self, other: &Range) -> bool {
            self.end + 1 == other.start // consider contiguous as end+1 == start
        }
    }

    struct CanonicalChecker {
        ranges: Vec<Range>,
    }

    impl CanonicalChecker {
        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0].start >= pair[1].start {
                    return false;
                }
                if pair[0].is_contiguous(&pair[1]) {
                    return false;
                }
            }
            true
        }
    }

    // Test case where two ranges are equal, should return false
    let ranges = vec![
        Range { start: 2, end: 4 },
        Range { start: 2, end: 4 }, // equal
    ];
    
    let checker = CanonicalChecker { ranges };
    assert!(!checker.is_canonical());
}

