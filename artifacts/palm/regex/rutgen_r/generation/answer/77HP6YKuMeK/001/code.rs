// Answer 0

#[test]
fn test_intervals_with_non_empty_ranges() {
    struct RangeSet {
        ranges: Vec<i32>,
    }

    impl RangeSet {
        pub fn intervals(&self) -> &[i32] {
            &self.ranges
        }
    }

    let range_set = RangeSet {
        ranges: vec![1, 2, 3, 4],
    };

    let result = range_set.intervals();
    assert_eq!(result, &[1, 2, 3, 4]);
}

#[test]
fn test_intervals_with_empty_ranges() {
    struct RangeSet {
        ranges: Vec<i32>,
    }

    impl RangeSet {
        pub fn intervals(&self) -> &[i32] {
            &self.ranges
        }
    }

    let range_set = RangeSet {
        ranges: vec![],
    };

    let result = range_set.intervals();
    assert_eq!(result, &[]);
}

#[test]
fn test_intervals_with_single_element() {
    struct RangeSet {
        ranges: Vec<i32>,
    }

    impl RangeSet {
        pub fn intervals(&self) -> &[i32] {
            &self.ranges
        }
    }

    let range_set = RangeSet {
        ranges: vec![42],
    };

    let result = range_set.intervals();
    assert_eq!(result, &[42]);
}

