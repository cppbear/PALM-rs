// Answer 0

#[test]
fn test_iter_empty_intervals() {
    struct IntervalSet {
        ranges: Vec<(i32, i32)>,
    }

    impl IntervalSet {
        pub fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.ranges.iter()
        }
    }

    let intervals = IntervalSet { ranges: vec![] };
    let mut iter = intervals.iter();
    
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_interval() {
    struct IntervalSet {
        ranges: Vec<(i32, i32)>,
    }

    impl IntervalSet {
        pub fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.ranges.iter()
        }
    }

    let intervals = IntervalSet { ranges: vec![(1, 5)] };
    let mut iter = intervals.iter();
    
    assert_eq!(iter.next(), Some(&(1, 5)));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_intervals() {
    struct IntervalSet {
        ranges: Vec<(i32, i32)>,
    }

    impl IntervalSet {
        pub fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.ranges.iter()
        }
    }

    let intervals = IntervalSet { ranges: vec![(1, 3), (4, 6), (7, 10)] };
    let mut iter = intervals.iter();
    
    assert_eq!(iter.next(), Some(&(1, 3)));
    assert_eq!(iter.next(), Some(&(4, 6)));
    assert_eq!(iter.next(), Some(&(7, 10)));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_overlapping_intervals() {
    struct IntervalSet {
        ranges: Vec<(i32, i32)>,
    }

    impl IntervalSet {
        pub fn iter(&self) -> std::slice::Iter<(i32, i32)> {
            self.ranges.iter()
        }
    }

    let intervals = IntervalSet { ranges: vec![(1, 5), (3, 7), (8, 10)] };
    let mut iter = intervals.iter();
    
    assert_eq!(iter.next(), Some(&(1, 5)));
    assert_eq!(iter.next(), Some(&(3, 7)));
    assert_eq!(iter.next(), Some(&(8, 10)));
    assert!(iter.next().is_none());
}

