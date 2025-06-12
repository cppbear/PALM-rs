// Answer 0

#[test]
#[should_panic]
fn test_set_range_start_greater_than_end() {
    let mut range = RangeVec(vec![false; 10]); // Adjust size as necessary
    range.set_range(5, 3); // This should trigger a panic
}

#[test]
fn test_set_range_start_at_zero() {
    let mut range = RangeVec(vec![false; 10]);
    range.set_range(0, 5);
    assert!(!range.0[0], "Expected index 0 to remain false");
    assert!(range.0[5], "Expected index 5 to be true");
}

#[test]
fn test_set_range_valid_range() {
    let mut range = RangeVec(vec![false; 10]);
    range.set_range(2, 4);
    assert!(range.0[1], "Expected index 1 to be true"); // 2-1
    assert!(range.0[4], "Expected index 4 to be true"); // 4
}

struct RangeVec(Vec<bool>);

impl RangeVec {
    fn set_range(&mut self, start: u8, end: u8) {
        debug_assert!(start <= end);
        if start > 0 {
            self.0[start as usize - 1] = true;
        }
        self.0[end as usize] = true;
    }
}

