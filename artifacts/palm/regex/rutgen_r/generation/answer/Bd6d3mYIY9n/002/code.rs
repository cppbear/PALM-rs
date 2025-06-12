// Answer 0

#[test]
fn test_is_canonical_with_non_canonical_ranges() {
    struct Range {
        start: usize,
        end: usize,
    }

    impl Range {
        fn new(start: usize, end: usize) -> Self {
            Range { start, end }
        }

        fn is_contiguous(&self, other: &Range) -> bool {
            self.end + 1 == other.start || other.end + 1 == self.start
        }
    }

    struct Ranges {
        ranges: Vec<Range>,
    }

    impl Ranges {
        fn new(ranges: Vec<Range>) -> Self {
            Ranges { ranges }
        }

        fn is_canonical(&self) -> bool {
            for pair in self.ranges.windows(2) {
                if pair[0].end >= pair[1].start {
                    return false;
                }
                if pair[0].is_contiguous(&pair[1]) {
                    return false;
                }
            }
            true
        }
    }

    let ranges = Ranges::new(vec![
        Range::new(1, 3),
        Range::new(3, 5), // non-canonical pair: they are contiguous
        Range::new(6, 8),
    ]);

    assert!(!ranges.is_canonical());
}

