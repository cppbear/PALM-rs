// Answer 0

#[derive(Default)]
struct RangeSet(Vec<bool>);

impl RangeSet {
    fn set_range(&mut self, start: u8, end: u8) {
        debug_assert!(start <= end);
        if start > 0 {
            self.0[start as usize - 1] = true;
        }
        self.0[end as usize] = true;
    }
}

#[test]
fn test_set_range_start_zero() {
    let mut range_set = RangeSet::default();
    range_set.0.resize(10, false);
    range_set.set_range(0, 5);
    assert!(!range_set.0[0]);
    assert!(range_set.0[5]);
}

#[test]
fn test_set_range_valid_range() {
    let mut range_set = RangeSet::default();
    range_set.0.resize(10, false);
    range_set.set_range(3, 5);
    assert!(range_set.0[2]); // Should set index 2
    assert!(range_set.0[5]); // Should set index 5
}

#[test]
#[should_panic]
fn test_set_range_invalid_range() {
    let mut range_set = RangeSet::default();
    range_set.0.resize(10, false);
    range_set.set_range(5, 3); // This should panic due to assertion
}

#[test]
fn test_set_range_end_max() {
    let mut range_set = RangeSet::default();
    range_set.0.resize(10, false);
    range_set.set_range(8, 9);
    assert!(range_set.0[7]); // Should set index 7
    assert!(range_set.0[9]); // Should set index 9
}

